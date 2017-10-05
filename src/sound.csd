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
  iattack = 0.5
  ; Decay time.
  idecay = 0.5
  ; Sustain level.
  isustain = 0.1
  ; Release time.
  irelease = 1.0
  aenv madsr iattack, idecay, isustain, irelease

  aSource vco2 p4, p5
  adl = 0.1
  aL, aR freeverb aSource, aSource, 0.5, 0.35, sr, 0
  outs aL*aenv, aR*aenv
endin

instr 2
  aLFO vco2 10, 500
  al2 vco2 1, 4
  aenv madsr 1.0, 0.1, 0.1, 0.1
  aSource poscil 1, aLFO*10 + p5
  afilter reson aSource, p5, 1
  asig = afilter*aenv
  aL, aR freeverb asig, asig, 0.8, 0.35, sr, 0
  outs aL, aR
endin


;gifn	ftgen	0,0, 257, 9, .5,1,270	
gifn	ftgen	0,0, 257, 9, .5,1,270,1.5,.33,90,2.5,.2,270,3.5,.143,90,4.5,.111,270

instr 3

  kpcs = p5
  kcar = 1
  kmod = p5
  kndx line 0, 0.1, 1.0	
  aLFO vco2 1, 10
  aFM vco2 1, p6
  as vco2 1+k(aLFO), k(aFM*1000)+p5
  af foscil 1, kpcs, p4, p5/100, kndx
  asource distort af+as, 0.9, gifn
  knum init 1.00
  knum = knum +0.1
  k2 = (aLFO + 2.00)/2.0
  aenv madsr 0.5, 0.9, 0.1, 1.0
  aL, aR freeverb asource, asource, 0.91, 0.1, sr, 1
  outs aL*asource*aenv, aR*asource*aenv
endin

</CsInstruments>

<CsScore>
f1 0 32768 10 1

i3 0 5 1 400 100
i3 5 2 1 500 200
i3 8 3 1 1000 560

</CsScore>

</CsoundSynthesizer>
