/* Twofish (128-Bit-Schlüssel) + CFB1-Modus wie in der Twofish-Referenzimplementierung,
   die in IMPORT.DLL eingebettet ist (makeKey 0x101164e0, cipherInit 0x10116560,
   blockEncrypt 0x101165c0, blockDecrypt 0x101173c0). q-Tabellen aus IMPORT.DLL. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "tfq.h"
typedef uint32_t u32; typedef uint8_t u8;
static u32 K[40]; static u32 S[2];
static u8 gfmul(u8 a,u8 b,unsigned poly){unsigned r=0,x=a;while(b){if(b&1)r^=x;x<<=1;if(x&0x100)x^=poly;b>>=1;}return r;}
static const u8 MDS[4][4]={{0x01,0xEF,0x5B,0x5B},{0x5B,0xEF,0xEF,0x01},{0xEF,0x5B,0x01,0xEF},{0xEF,0x01,0xEF,0x5B}};
static const u8 RS[4][8]={{0x01,0xA4,0x55,0x87,0x5A,0x58,0xDB,0x9E},{0xA4,0x56,0x82,0xF3,0x1E,0xC6,0x68,0xE5},{0x02,0xA1,0xFC,0xC1,0x47,0xAE,0x3D,0x19},{0xA4,0x55,0x87,0x5A,0x58,0xDB,0x9E,0x03}};
#define ROL(x,n) (((x)<<(n))|((x)>>(32-(n))))
#define ROR(x,n) (((x)>>(n))|((x)<<(32-(n))))
static u32 h(u32 X,const u32*L){ u8 y[4]; for(int i=0;i<4;i++)y[i]=(X>>(8*i))&0xff;
 u8 l0[4],l1[4]; for(int i=0;i<4;i++){l0[i]=(L[0]>>(8*i))&0xff;l1[i]=(L[1]>>(8*i))&0xff;}
 y[0]=Q1[Q0[Q0[y[0]]^l1[0]]^l0[0]]; y[1]=Q0[Q0[Q1[y[1]]^l1[1]]^l0[1]];
 y[2]=Q1[Q1[Q0[y[2]]^l1[2]]^l0[2]]; y[3]=Q0[Q1[Q1[y[3]]^l1[3]]^l0[3]];
 u32 z=0; for(int i=0;i<4;i++){u8 s=0; for(int j=0;j<4;j++) s^=gfmul(MDS[i][j],y[j],0x169); z|=(u32)s<<(8*i);} return z;}
static void tf_setkey(const u8*k){ u32 M[4]; for(int i=0;i<4;i++)M[i]=k[4*i]|k[4*i+1]<<8|k[4*i+2]<<16|(u32)k[4*i+3]<<24;
 u32 Me[2]={M[0],M[2]},Mo[2]={M[1],M[3]};
 for(int i=0;i<2;i++){ u8 s[4]; for(int r=0;r<4;r++){u8 v=0;for(int c=0;c<8;c++)v^=gfmul(RS[r][c],k[8*i+c],0x14D);s[r]=v;}
   S[1-i]=s[0]|s[1]<<8|s[2]<<16|(u32)s[3]<<24; }
 for(int i=0;i<20;i++){ u32 A=h(2*i*0x01010101u,Me), B=ROL(h((2*i+1)*0x01010101u,Mo),8); K[2*i]=A+B; K[2*i+1]=ROL(A+2*B,9);} }
static void enc(const u8*in,u8*out){ u32 R[4]; for(int i=0;i<4;i++)R[i]=(in[4*i]|in[4*i+1]<<8|in[4*i+2]<<16|(u32)in[4*i+3]<<24)^K[i];
 for(int r=0;r<16;r++){ u32 T0=h(R[0],S),T1=h(ROL(R[1],8),S); u32 F0=T0+T1+K[2*r+8],F1=T0+2*T1+K[2*r+9];
  R[2]=ROR(R[2]^F0,1); R[3]=ROL(R[3],1)^F1; u32 a=R[0],b=R[1]; R[0]=R[2];R[1]=R[3];R[2]=a;R[3]=b; }
 u32 C[4]={R[2]^K[4],R[3]^K[5],R[0]^K[6],R[1]^K[7]};
 for(int i=0;i<4;i++)for(int j=0;j<4;j++)out[4*i+j]=(C[i]>>(8*j))&0xff; }
static int hex2(const char*s,u8*o){for(int i=0;i<16;i++){unsigned v;if(sscanf(s+2*i,"%2x",&v)!=1)return 0;o[i]=v;}return 1;}
int main(int argc,char**argv){
 if(argc==2&&!strcmp(argv[1],"test")){u8 k[16]={0},p[16]={0},c[16];tf_setkey(k);enc(p,c);for(int i=0;i<16;i++)printf("%02X",c[i]);printf("  (soll 9F589F5CF6122C32B6BFEC2F2AE8C35A)\n");return 0;}
 /* usage: twofish_cfb1 KEYHEX IVHEX infile outfile [maxbytes] */
 u8 key[16],iv[16]; hex2(argv[1],key); hex2(argv[2],iv); tf_setkey(key);
 FILE*f=fopen(argv[3],"rb"); fseek(f,0,2); long n=ftell(f); fseek(f,0,0); u8*in=malloc(n),*out=malloc(n); fread(in,1,n,f); fclose(f);
 if(argc>5){long m=atol(argv[5]); if(m<n)n=m;}
 for(long bitn=0;bitn<n*8;bitn++){ u8 x[16]; enc(iv,x); int sh=bitn&7; u8 bit0=0x80>>sh;
   u8 ctBit=in[bitn/8]&bit0;                 /* Entschlüsseln: Chiffretext-Bit geht ins IV */
   u8 ptBit=ctBit ^ ((x[0]&0x80)>>sh);
   out[bitn/8]=(out[bitn/8]&~bit0)|ptBit;
   u8 carry=ctBit>>(7-sh); for(int i=15;i>=0;i--){u8 b=iv[i]>>7; iv[i]=(iv[i]<<1)^carry; carry=b;} }
 FILE*g=fopen(argv[4],"wb"); fwrite(out,1,n,g); fclose(g); return 0; }
