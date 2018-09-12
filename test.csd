<CsoundSynthesizer>
<CsOptions>
</CsOptions>
<CsInstruments>


sr = 44100
ksmps = 32
nchnls = 2
0dbfs = 1

massign   0, "Egg" ;assign all MIDI channels to instrument 1

instr Egg
iCps    cpsmidi   ;get the frequency from the key pressed
iCps *= 4
iAmp    ampmidi   0dbfs * 0.3 ;get the amplitude
aOut    poscil    iAmp, iCps ;generate a sine tone
        outs      aOut, aOut ;write it to the output
endin

</CsInstruments>
<CsScore>
f0 360000
; f1 0 16384 10 1 
; f2 0 16 7 1 8 0 8
; f3 0 1024 10 1 .5 .6 .3 .2 .5

; e
</CsScore>
</CsoundSynthesizer>

