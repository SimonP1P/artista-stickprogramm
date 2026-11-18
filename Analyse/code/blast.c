/* PKWARE DCL explode, nach dem Algorithmus von Mark Adler (zlib contrib/blast) */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#define MAXBITS 13
#define MAXWIN 4096
typedef struct { const unsigned char *in; size_t left; int bitbuf, bitcnt; jmp_buf env;
 unsigned char *out; size_t outlen, outcap; } state;
static int bits(state *s, int need){ int val=s->bitbuf;
 while(s->bitcnt<need){ if(s->left==0) longjmp(s->env,1); val|=(int)(*s->in++)<<s->bitcnt; s->left--; s->bitcnt+=8;}
 s->bitbuf=val>>need; s->bitcnt-=need; return val&((1<<need)-1);}
struct huffman { short *count; short *symbol; };
static int decode(state *s, struct huffman *h){ int len=1,code=0,first=0,count,index=0,bitbuf=s->bitbuf,left=s->bitcnt; short *next=h->count+1;
 while(1){ while(left--){ code|=(bitbuf&1)^1; bitbuf>>=1; count=*next++;
   if(code<first+count){ s->bitbuf=bitbuf; s->bitcnt=(s->bitcnt-len)&7; return h->symbol[index+(code-first)];}
   index+=count; first+=count; first<<=1; code<<=1; len++; }
  left=(MAXBITS+1)-len; if(left==0) break; if(s->left==0) longjmp(s->env,1);
  bitbuf=*s->in++; s->left--; if(left>8) left=8; }
 return -9; }
static int construct(struct huffman *h, const unsigned char *rep, int n){ int symbol=0,len,left; short offs[MAXBITS+1]; short length[256];
 do{ len=*rep++; left=(len>>4)+1; len&=15; do{ length[symbol++]=len; }while(--left); }while(--n);
 n=symbol; for(len=0;len<=MAXBITS;len++) h->count[len]=0; for(symbol=0;symbol<n;symbol++) h->count[length[symbol]]++;
 if(h->count[0]==n) return 0; left=1; for(len=1;len<=MAXBITS;len++){ left<<=1; left-=h->count[len]; if(left<0) return left;}
 offs[1]=0; for(len=1;len<MAXBITS;len++) offs[len+1]=offs[len]+h->count[len];
 for(symbol=0;symbol<n;symbol++) if(length[symbol]!=0) h->symbol[offs[length[symbol]]++]=symbol; return left; }
static void put(state *s, unsigned char c){ if(s->outlen==s->outcap){ s->outcap=s->outcap?s->outcap*2:65536; s->out=realloc(s->out,s->outcap);} s->out[s->outlen++]=c; }
static int decomp(state *s){ int lit,dict,symbol,len; unsigned dist; int copy;
 static const unsigned char litlen[]={11,124,8,7,28,7,188,13,76,4,10,8,12,10,12,10,8,23,8,9,7,6,7,8,7,6,55,8,23,24,12,11,7,9,11,12,6,7,22,5,7,24,6,11,9,6,7,22,7,11,38,7,9,8,25,11,8,11,9,12,8,12,5,38,5,38,5,11,7,5,6,21,6,10,53,8,7,24,10,27,44,253,253,253,252,252,252,13,12,45,12,45,12,61,12,45,44,173};
 static const unsigned char lenlen[]={2,35,36,53,38,23};
 static const unsigned char distlen[]={2,20,53,230,247,151,248};
 static const short base[16]={3,2,4,5,6,7,8,9,10,12,16,24,40,72,136,264};
 static const char extra[16]={0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8};
 static int virgin=1; static short litcnt[MAXBITS+1],litsym[256],lencnt[MAXBITS+1],lensym[16],distcnt[MAXBITS+1],distsym[64];
 static struct huffman litcode={litcnt,litsym},lencode={lencnt,lensym},distcode={distcnt,distsym};
 if(virgin){ construct(&litcode,litlen,sizeof(litlen)); construct(&lencode,lenlen,sizeof(lenlen)); construct(&distcode,distlen,sizeof(distlen)); virgin=0;}
 lit=bits(s,8); if(lit>1) return -1; dict=bits(s,8); if(dict<4||dict>6) return -2;
 do{ if(bits(s,1)){ symbol=decode(s,&lencode); len=base[symbol]+bits(s,extra[symbol]); if(len==519) break;
   symbol=len==2?2:dict; dist=decode(s,&distcode)<<symbol; dist+=bits(s,symbol); dist++;
   if(dist>s->outlen) return -3;
   for(copy=0;copy<len;copy++) put(s,s->out[s->outlen-dist]);
  } else { symbol=lit?decode(s,&litcode):bits(s,8); put(s,(unsigned char)symbol);} }while(1);
 return 0; }
int main(int argc,char**argv){ /* stdin: compressed, stdout: decompressed */
 size_t cap=1<<20,n=0; unsigned char*buf=malloc(cap); size_t r;
 while((r=fread(buf+n,1,cap-n,stdin))>0){ n+=r; if(n==cap){cap*=2;buf=realloc(buf,cap);} }
 state s; memset(&s,0,sizeof s); s.in=buf; s.left=n;
 int err; if(setjmp(s.env)) err=2; else err=decomp(&s);
 fwrite(s.out,1,s.outlen,stdout); if(err) fprintf(stderr,"blast err %d\n",err); return err?1:0; }
