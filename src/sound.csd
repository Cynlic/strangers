<CsoundSynthesizer>

<CsOptions>
</CsOptions>
<CsInstruments>
sr = 44100
ksmps = 32
nchnls = 2
0dbfs  = 1

instr 1
  ; Attack time.
  iattack = 0.1
  ; Decay time.
  idecay = 0.01
  ; Sustain level.
  isustain = 0.001
  ; Release time.
  irelease = 0.01
  idelay = p5
  aenv madsr iattack, idecay, isustain, irelease

  aSource vco2 p4/2.0, p5
  as2 vco2 p4/2.0, p5*3/2.0
  as3 vco2 p4/2.0, (p5*6/2.0)
  afin = ((aSource + as2+as3) * aenv)
  aL, aR freeverb afin, afin, 0.5, 0.9, sr, 0
  adl1 vdelay aL, idelay+(p3*100), 5000
  adl2 vdelay adl1, idelay+(p3*100), 5000
  outs aL+adl1+adl2, aR+adl1+adl2
endin

instr 4

  krd rand 1, 10
  krand abs krd
  ; Attack time.
  iattack = 0.8 -  i(krand) 
  ; Decay time.
  idecay = 0.02 + i(krand)
  ; Sustain level.
  isustain = 0.0001
  ; Release time.
  irelease = 0.001
  ; Delay line control
  idelay = p5 +i(krand)

  aenv madsr iattack, idecay, isustain, irelease

  aSource vco2 p4/2.0, p5
  as2 vco2 p4/2.0, p5*3/2.0
  as3 vco2 p4/2.0, (p5*krand/2.0)
  as4 vco2 p4/2.0, (p5*3/(2.0+krand))
  afin = ((aSource+as2+as3+as4) * aenv)
  aL, aR freeverb afin, afin, 0.5, 0.9, sr, 0
  adl1 vdelay aL, idelay+(p3*100), 5000
  adl2 vdelay adl1, idelay+(p3*100), 5000
  adl3 vdelay adl2, idelay+(p3*100), 5000
  adl4 vdelay adl3, idelay+(p3*100), 5000
  adl5 vdelay adl4, idelay+(p3*100), 5000
  outs aL+adl1+adl2, aR+adl3+adl4
endin


instr 2
  aLFO vco2 1, 1000
  aenv madsr p3/10.0, 0.01, 0.010, 1.0
  kfo = k(aLFO)
  aSource poscil 1,(kfo+1)*100 + p5
  as2     poscil 1, (kfo+1)*100 + (p5*4)
  as3     poscil 1, (kfo+1)*10 + (p5*8/3)
  as4     poscil 1, (kfo+1)*10 + (p5*8/5)
  as = as2+as3+as4
  afilter butterbp aSource, p2*2.0, 100
  af2     butterbp as, p2*2.0, p3
  asig = (af2+afilter) * aenv
  aL, aR freeverb asig, asig, 0.8, 0.95, sr, 0
  a1, a2 pan2 aL, 0.0
  outs a1*2.0, a2*2.0
endin


instr 5

  aLFO vco2 1, 4
  aenv madsr 2.0, 0.1, 0.00, 0.5
  kfo = k(aLFO)
  aSource poscil 1,(kfo+1)*100 + p5
  as2     poscil 1, (kfo+1)*100 + (p5*4)
  as3     poscil 1, (kfo+1)*10 + (p5*8/3)
  as4     poscil 1, (kfo+1)*10 + (p5*8/5)
  as = as2+as3+as4
  afilter butterbp aSource, p2*2.0, 100
  af2     butterbp as, p2*2.0, p3
  asig = (as - afilter +af2) * aenv * aLFO
  aL, aR freeverb asig, asig, 0.8, 0.95, sr, 0
  a1, a2 pan2 aL, 0.0
  outs a1*2.0, a2*2.0
endin


gifn	ftgen	0,0, 257, 9, .5,1,270,1.5,.33,90,2.5,.2,270,3.5,.143,90,4.5,.111,270

instr 3

  kpcs = 440+p5
  kcar = 1
  kmod = p3 * 1001
  kndx line 0, 0.1, 1.0
  aLFO vco2 1, 10
  aFM vco2 1, p5
  afm2 vco2 1, p5* 3.0/2.0
  as vco2 1+k(aLFO), k(aFM*1000)+p5
  af foscil p4/5.0, kpcs, kcar, kmod, kndx
  asource distort af+aFM+afm2, 0.99, gifn
  knum init 1.00
  knum = knum +0.1
  k2 = (aLFO + 2.00)/2.0
  aenv madsr p3, 0.1, 0.00, 2.0
  afin = aenv*asource
  aL, aR freeverb afin, afin, 0.91, 0.8, sr, 0
  a1, a2 pan2 aL, 1.0
  outs a1/5.0, a2/5.0
endin


</CsInstruments>

<CsScore>
f1 0 32768 10

i 3 -0.33366565718097974 7.235373783857477 -0.9461838427664065 160
i 2  3.76606434638973 10.034889064416188 -0.8633183854692369 86
i 1 9.690925755309047 5.260551392540384 -0.9132561674303454 14
i 4 10.46681174272038 4.378920076784422 -0.9944577798083436 159 
i 3 11.17785637032071 8.095837900661488 -0.9205333710903775 188 
i 5 16.179126199939315 12.299759450249972 -0.8622574290020399 148 
i 1 22.490093735072517 13.283817765610289 -0.9388310216294345 111 
i 3 26.298923793690715 12.2653993704365 -0.9908079262418843 68 
i 5 30.99618747328052 7.823451584254417 -0.8966056125117785 14 
i 4 36.23901207445518 5.163469992843608 -0.8686416066621684 60 
i 2 38.739903619427274 4.362991539907551 -0.9631279960458384 94 
i 3 38.615971348397075 2.9472459980894676 -0.9775253478543396 61 
i 1 39.97146795886198 0.7637306969340203 -0.8778460273766086 84 
i 2 42.29231002310518 2.628352186917128 -0.8824651245563088 97 
i 4 42.909289825999934 1.3763385303762345 -0.9821316023324835 63 
i 1 41.68801240439804 1.9695928295291936 -0.9566613071846686 150 
i 1 43.26893987114904 7.6990564869398534 -0.8661135232804708 111 
i 5 48.36613258742127 4.294432858094135 -0.9029506519039816 24 
i 1 48.98168511064475 2.3741251472404037 -0.9928215178692016 174 
i 4 48.6045514478897 8.845907430547848 -0.9315817073790095 171 
i 1 53.713730948216394 11.953131126589891 -0.8618578870225919 120 
i 3 60.15290564450925 11.28294087240092 -0.9277551124313224 65 
i 1 63.24572994577829 5.627379669915402 -0.9935687410836856 6 

</CsScore>

</CsoundSynthesizer>
