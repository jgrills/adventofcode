d = [ln.strip() for ln in open("long","rt")]; E = enumerate
for r,w in E(d):
  for c,h in E(w):
    if h=='S': START=(r,c); d[r] = d[r].replace('S','a')
    if h=='E': ENDPT=(r,c); d[r] = d[r].replace('E','z')

t = [[ord(h)-ord('a') for h in w] for w in d]
NR,NC = len(t),len(t[0])

def nbs(r,c): return ((r,c+1),(r+1,c),(r-1,c),(r,c-1))

def adv(pts,done):
  nxt = []
  for r,c in pts:
    for p,q in nbs(r,c):
      if not(0<=p<NR and 0<=q<NC) or t[p][q]>t[r][c]+1: continue
      if (pq:=(p,q)) in done: continue
      if pq==ENDPT: return True,()
      nxt.append(pq); done.add(pq)
  return False,nxt

def slv(start,m=500):
  pts = [start]; done = set([start])
  for n in range(m):
    ended,pts = adv(pts,done)
    if ended: return n+1
  return m

print(slv(START), # 449 443
      min(slv((r,c)) for r,w in E(t) for c,h in E(w) if h==0))
