# html/semantics/embedded-content/the-img-element/sizes/support/sizes-iframed.sub.html

Counts:
- errors: 1
- warnings: 293
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/sizes/support/sizes-iframed.sub.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!{{GET[doctype]}}>
<style> img { {{GET[style]}} } </style>
<!-- First <img> in a <p> is the reference. The following <img>s should be equivalent -->
<!-- default is 100vw, not 300px -->
<p>
<img srcset='/images/green-1x1.png?a1 300w, /images/green-16x16.png?a1 301w' sizes='100vw'>
<img srcset='/images/green-1x1.png?a2 300w, /images/green-16x16.png?a2 301w'>
<p>
<img srcset='/images/green-1x1.png?b1 450w, /images/green-16x16.png?b1 451w' sizes='100vw'>
<img srcset='/images/green-1x1.png?b2 450w, /images/green-16x16.png?b2 451w'>
<p>
<img srcset='/images/green-1x1.png?c1 600w, /images/green-16x16.png?c1 601w' sizes='100vw'>
<img srcset='/images/green-1x1.png?c2 600w, /images/green-16x16.png?c2 601w'>
<p>
<img srcset='/images/green-1x1.png?d1 900w, /images/green-16x16.png?d1 901w' sizes='100vw'>
<img srcset='/images/green-1x1.png?d2 900w, /images/green-16x16.png?d2 901w'>

<p>
<img srcset='/images/green-1x1.png?e1 50w, /images/green-16x16.png?e1 51w' sizes='1px'>
<img srcset='/images/green-1x1.png?e2 50w, /images/green-16x16.png?e2 51w' sizes='0'>
<img srcset='/images/green-1x1.png?e3 50w, /images/green-16x16.png?e3 51w' sizes='-0'>
<img srcset='/images/green-1x1.png?e4 50w, /images/green-16x16.png?e4 51w' sizes='+0'>
<img srcset='/images/green-1x1.png?e5 50w, /images/green-16x16.png?e5 51w' sizes='+1px'>
<img srcset='/images/green-1x1.png?e6 50w, /images/green-16x16.png?e6 51w' sizes='.1px'>
<img srcset='/images/green-1x1.png?e7 50w, /images/green-16x16.png?e7 51w' sizes='0.1em'>
<img srcset='/images/green-1x1.png?e8 50w, /images/green-16x16.png?e8 51w' sizes='0.1ex'>
<img srcset='/images/green-1x1.png?e9 50w, /images/green-16x16.png?e9 51w' sizes='0.1ch'>
<img srcset='/images/green-1x1.png?e10 50w, /images/green-16x16.png?e10 51w' sizes='0.1rem'>
<img srcset='/images/green-1x1.png?e11 50w, /images/green-16x16.png?e11 51w' sizes='0.1vw'>
<img srcset='/images/green-1x1.png?e12 50w, /images/green-16x16.png?e12 51w' sizes='0.1vh'>
<img srcset='/images/green-1x1.png?e13 50w, /images/green-16x16.png?e13 51w' sizes='0.1vmin'>
<img srcset='/images/green-1x1.png?e14 50w, /images/green-16x16.png?e14 51w' sizes='0.1vmax'>
<img srcset='/images/green-1x1.png?e15 50w, /images/green-16x16.png?e15 51w' sizes='0.1cm'>
<img srcset='/images/green-1x1.png?e16 50w, /images/green-16x16.png?e16 51w' sizes='1mm'>
<img srcset='/images/green-1x1.png?e17 50w, /images/green-16x16.png?e17 51w' sizes='1q'>
<img srcset='/images/green-1x1.png?e18 50w, /images/green-16x16.png?e18 51w' sizes='0.01in'>
<img srcset='/images/green-1x1.png?e19 50w, /images/green-16x16.png?e19 51w' sizes='0.1pc'>
<img srcset='/images/green-1x1.png?e20 50w, /images/green-16x16.png?e20 51w' sizes='0.1pt'>
<img srcset='/images/green-1x1.png?e21 50w, /images/green-16x16.png?e21 51w' sizes='/* */1px/* */'>
<img srcset='/images/green-1x1.png?e22 50w, /images/green-16x16.png?e22 51w' sizes=' /**/ /**/ 1px /**/ /**/ '>
<img srcset='/images/green-1x1.png?e23 50w, /images/green-16x16.png?e23 51w' sizes='(),1px'>
<img srcset='/images/green-1x1.png?e24 50w, /images/green-16x16.png?e24 51w' sizes='x(),1px'>
<img srcset='/images/green-1x1.png?e25 50w, /images/green-16x16.png?e25 51w' sizes='{},1px'>
<img srcset='/images/green-1x1.png?e26 50w, /images/green-16x16.png?e26 51w' sizes='[],1px'>
<img srcset='/images/green-1x1.png?e27 50w, /images/green-16x16.png?e27 51w' sizes='1px,('>
<img srcset='/images/green-1x1.png?e28 50w, /images/green-16x16.png?e28 51w' sizes='1px,x('>
<img srcset='/images/green-1x1.png?e29 50w, /images/green-16x16.png?e29 51w' sizes='1px,{'>
<img srcset='/images/green-1x1.png?e30 50w, /images/green-16x16.png?e30 51w' sizes='1px,['>
<img srcset='/images/green-1x1.png?e31 50w, /images/green-16x16.png?e31 51w' sizes='\(,1px'>
<img srcset='/images/green-1x1.png?e32 50w, /images/green-16x16.png?e32 51w' sizes='x\(,1px'>
<img srcset='/images/green-1x1.png?e33 50w, /images/green-16x16.png?e33 51w' sizes='\{,1px'>
<img srcset='/images/green-1x1.png?e34 50w, /images/green-16x16.png?e34 51w' sizes='\[,1px'>
<img srcset='/images/green-1x1.png?e35 50w, /images/green-16x16.png?e35 51w' sizes='1\p\x'>
<img srcset='/images/green-1x1.png?e36 50w, /images/green-16x16.png?e36 51w' sizes='calc(1px)'>
<img srcset='/images/green-1x1.png?e36a 50w, /images/green-16x16.png?e36a 51w' sizes='min(1px, 100px)'>
<img srcset='/images/green-1x1.png?e36b 50w, /images/green-16x16.png?e36b 51w' sizes='min(-100px, 1px)'>
<img srcset='/images/green-1x1.png?e37 50w, /images/green-16x16.png?e37 51w' sizes='(min-width:0) calc(1px)'>
<img srcset='/images/green-1x1.png?e37a 50w, /images/green-16x16.png?e37a 51w' sizes='(min-width:0) min(1px, 100px)'>
<img srcset='/images/green-1x1.png?e37b 50w, /images/green-16x16.png?e37b 51w' sizes='(min-width:0) max(-100px, 1px)'>
<img srcset='/images/green-1x1.png?e38 50w, /images/green-16x16.png?e38 51w' sizes='(min-width:calc(0)) 1px'>
<img srcset='/images/green-1x1.png?e39 50w, /images/green-16x16.png?e39 51w' sizes='(min-width:0) 1px, 100vw'>
<img srcset='/images/green-1x1.png?e40 50w, /images/green-16x16.png?e40 51w' sizes='(min-width:0) 1px, (min-width:0) 100vw, 100vw'>
<img srcset='/images/green-1x1.png?e41 50w, /images/green-16x16.png?e41 51w' sizes='(min-width:0) 1px'>
<img srcset='/images/green-1x1.png?e42 50w, /images/green-16x16.png?e42 51w' sizes='not (min-width:0) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e43 50w, /images/green-16x16.png?e43 51w' sizes='(min-width:unknown-mf-value) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e44 50w, /images/green-16x16.png?e44 51w' sizes='not (min-width:unknown-mf-value) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e45 50w, /images/green-16x16.png?e45 51w' sizes='(min-width:-1px) 1px, 100vw'>
<img srcset='/images/green-1x1.png?e46 50w, /images/green-16x16.png?e46 51w' sizes='not (min-width:-1px) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e47 50w, /images/green-16x16.png?e47 51w' sizes='(unknown-mf-name) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e48 50w, /images/green-16x16.png?e48 51w' sizes='not (unknown-mf-name) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e49 50w, /images/green-16x16.png?e49 51w' sizes='(unknown "general-enclosed") 100vw, 1px'>
<img srcset='/images/green-1x1.png?e50 50w, /images/green-16x16.png?e50 51w' sizes='not (unknown "general-enclosed") 100vw, 1px'>
<img srcset='/images/green-1x1.png?e51 50w, /images/green-16x16.png?e51 51w' sizes='unknown-general-enclosed(foo) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e52 50w, /images/green-16x16.png?e52 51w' sizes='not unknown-general-enclosed(foo) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e53 50w, /images/green-16x16.png?e53 51w' sizes='print 100vw, 1px'>
<img srcset='/images/green-1x1.png?e54 50w, /images/green-16x16.png?e54 51w' sizes='not print 100vw, 1px'>
<img srcset='/images/green-1x1.png?e55 50w, /images/green-16x16.png?e55 51w' sizes='unknown-media-type 100vw, 1px'>
<img srcset='/images/green-1x1.png?e56 50w, /images/green-16x16.png?e56 51w' sizes='not unknown-media-type 100vw, 1px'>
<img srcset='/images/green-1x1.png?e57 50w, /images/green-16x16.png?e57 51w' sizes='(min-width:0) or (min-width:0) 1px'>
<img srcset='/images/green-1x1.png?e58 50w, /images/green-16x16.png?e58 51w' sizes='(min-width:0) or (unknown-mf-name) 1px'>
<img srcset='/images/green-1x1.png?e59 50w, /images/green-16x16.png?e59 51w' sizes='(min-width:0) or (min-width:unknown-mf-value) 1px'>
<img srcset='/images/green-1x1.png?e60 50w, /images/green-16x16.png?e60 51w' sizes='(min-width:0) or (min-width:-1px) 1px'>
<img srcset='/images/green-1x1.png?e61 50w, /images/green-16x16.png?e61 51w' sizes='(min-width:0) or (unknown "general-enclosed") 1px'>
<img srcset='/images/green-1x1.png?e62 50w, /images/green-16x16.png?e62 51w' sizes='(min-width:0) or unknown-general-enclosed(foo) 1px'>
<img srcset='/images/green-1x1.png?e63 50w, /images/green-16x16.png?e63 51w' sizes='(min-width:0) or (]) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e64 50w, /images/green-16x16.png?e64 51w' sizes='(min-width:0) or unknown-media-type 100vw, 1px'>
<img srcset='/images/green-1x1.png?e65 50w, /images/green-16x16.png?e65 51w' sizes='(123) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e66 50w, /images/green-16x16.png?e66 51w' sizes='not (123) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e67 50w, /images/green-16x16.png?e67 51w' sizes='(!) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e68 50w, /images/green-16x16.png?e68 51w' sizes='not (!) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e69 50w, /images/green-16x16.png?e69 51w' sizes='! 100vw, 1px'>
<img srcset='/images/green-1x1.png?e70 50w, /images/green-16x16.png?e70 51w' sizes='not ! 100vw, 1px'>
<img srcset='/images/green-1x1.png?e71 50w, /images/green-16x16.png?e71 51w' sizes='(]) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e72 50w, /images/green-16x16.png?e72 51w' sizes='not (]) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e73 50w, /images/green-16x16.png?e73 51w' sizes='] 100vw, 1px'>
<img srcset='/images/green-1x1.png?e74 50w, /images/green-16x16.png?e74 51w' sizes='not ] 100vw, 1px'>
<img srcset='/images/green-1x1.png?e75 50w, /images/green-16x16.png?e75 51w' sizes='(}) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e76 50w, /images/green-16x16.png?e76 51w' sizes='not (}) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e77 50w, /images/green-16x16.png?e77 51w' sizes='} 100vw, 1px'>
<img srcset='/images/green-1x1.png?e78 50w, /images/green-16x16.png?e78 51w' sizes='not } 100vw, 1px'>
<img srcset='/images/green-1x1.png?e79 50w, /images/green-16x16.png?e79 51w' sizes=') 100vw, 1px'>
<img srcset='/images/green-1x1.png?e80 50w, /images/green-16x16.png?e80 51w' sizes='not ) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e81 50w, /images/green-16x16.png?e81 51w' sizes='(;) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e82 50w, /images/green-16x16.png?e82 51w' sizes='not (;) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e83 50w, /images/green-16x16.png?e83 51w' sizes='(.) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e84 50w, /images/green-16x16.png?e84 51w' sizes='not (.) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e85 50w, /images/green-16x16.png?e85 51w' sizes='; 100vw, 1px'>
<img srcset='/images/green-1x1.png?e86 50w, /images/green-16x16.png?e86 51w' sizes='not ; 100vw, 1px'>
<img srcset='/images/green-1x1.png?e87 50w, /images/green-16x16.png?e87 51w' sizes=', 1px'>
<img srcset='/images/green-1x1.png?e88 50w, /images/green-16x16.png?e88 51w' sizes='1px,'>
<img srcset='/images/green-1x1.png?e89 50w, /images/green-16x16.png?e89 51w' sizes='(min-width:0) 1px,'>
<img srcset='/images/green-1x1.png?e90 50w, /images/green-16x16.png?e90 51w' sizes='-0e-0px'>
<img srcset='/images/green-1x1.png?e91 50w, /images/green-16x16.png?e91 51w' sizes='+0.11e+01px'>
<img srcset='/images/green-1x1.png?e92 50w, /images/green-16x16.png?e92 51w' sizes='0.2e1px'>
<img srcset='/images/green-1x1.png?e93 50w, /images/green-16x16.png?e93 51w' sizes='0.3E1px'>
<img srcset='/images/green-1x1.png?e94 50w, /images/green-16x16.png?e94 51w' sizes='.4E1px'>
<img srcset='/images/green-1x1.png?e95 50w, /images/green-16x16.png?e95 51w' sizes='all 100vw, 1px'>
<img srcset='/images/green-1x1.png?e96 50w, /images/green-16x16.png?e96 51w' sizes='all and (min-width:0) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e97 50w, /images/green-16x16.png?e97 51w' sizes='min-width:0 100vw, 1px'>
<img srcset='/images/green-1x1.png?e98 50w, /images/green-16x16.png?e98 51w' sizes='1px, 100vw'>
<img srcset='/images/green-1x1.png?e99 50w, /images/green-16x16.png?e99 51w' sizes='1px, (min-width:0) 100vw'>
<img srcset='/images/green-1x1.png?e100 50w, /images/green-16x16.png?e100 51w' sizes='1px, foo bar'>
<img srcset='/images/green-1x1.png?e101 50w, /images/green-16x16.png?e101 51w' sizes='(min-width:0) 1px, foo bar'>
<img srcset='/images/green-1x1.png?e102 50w, /images/green-16x16.png?e102 51w' sizes='("grammar does not match") 100vw, 1px'>
<img srcset='/images/green-1x1.png?e103 50w, /images/green-16x16.png?e103 51w' sizes='not ("grammar does not match") 100vw, 1px'>
<img srcset='/images/green-1x1.png?e104 50w, /images/green-16x16.png?e104 51w' sizes='(unknown-general-enclosed !) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e105 50w, /images/green-16x16.png?e105 51w' sizes='not (unknown-general-enclosed !) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e106 50w, /images/green-16x16.png?e106 51w' sizes='(min-width:0) or (unknown-general-enclosed !) 1px'>
<img srcset='/images/green-1x1.png?e107 50w, /images/green-16x16.png?e107 51w' sizes='not ((min-width:0) or (unknown "general-enclosed")) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e108 50w, /images/green-16x16.png?e108 51w' sizes='(max-width:0) or (unknown-general-enclosed !) 100vw, 1px'>
<img srcset='/images/green-1x1.png?e109 50w, /images/green-16x16.png?e109 51w' sizes='not ((max-width:0) or (unknown "general-enclosed")) 100vw, 1px'>
<img srcset='/images/green-1x1.png?f48 50w, /images/green-16x16.png?f48 51w' sizes='calc(1px'>
<img srcset='/images/green-1x1.png?f48a 50w, /images/green-16x16.png?f48a 51w' sizes='min(1px, 200vw'>
<img srcset='/images/green-1x1.png?f48b 50w, /images/green-16x16.png?f48b 51w' sizes='max(-200vw, 1px'>
<img srcset='/images/green-1x1.png?f49 50w, /images/green-16x16.png?f49 51w' sizes='(min-width:0) calc(1px'>
<img srcset='/images/green-1x1.png?f49a 50w, /images/green-16x16.png?f49a 51w' sizes='(min-width:0) min(1px, 200vw'>
<img srcset='/images/green-1x1.png?f49b 50w, /images/green-16x16.png?f49b 51w' sizes='(min-width:0) max(-200vw, 1px'>

<p>
<img srcset='/images/green-1x1.png?f1 50w, /images/green-16x16.png?f1 51w' sizes='100vw'>
<img srcset='/images/green-1x1.png?f2 50w, /images/green-16x16.png?f2 51w' sizes=''>
<img srcset='/images/green-1x1.png?f3 50w, /images/green-16x16.png?f3 51w' sizes=','>
<img srcset='/images/green-1x1.png?f4 50w, /images/green-16x16.png?f4 51w' sizes='-1px'>
<img srcset='/images/green-1x1.png?f5 50w, /images/green-16x16.png?f5 51w' sizes='1'>
<img srcset='/images/green-1x1.png?f6 50w, /images/green-16x16.png?f6 51w' sizes='0.1%'>
<img srcset='/images/green-1x1.png?f7 50w, /images/green-16x16.png?f7 51w' sizes='0.1deg'>
<img srcset='/images/green-1x1.png?f8 50w, /images/green-16x16.png?f8 51w' sizes='0.1grad'>
<img srcset='/images/green-1x1.png?f9 50w, /images/green-16x16.png?f9 51w' sizes='0.1rad'>
<img srcset='/images/green-1x1.png?f10 50w, /images/green-16x16.png?f10 51w' sizes='0.1turn'>
<img srcset='/images/green-1x1.png?f11 50w, /images/green-16x16.png?f11 51w' sizes='0.1s'>
<img srcset='/images/green-1x1.png?f12 50w, /images/green-16x16.png?f12 51w' sizes='0.1ms'>
<img srcset='/images/green-1x1.png?f13 50w, /images/green-16x16.png?f13 51w' sizes='0.1Hz'>
<img srcset='/images/green-1x1.png?f14 50w, /images/green-16x16.png?f14 51w' sizes='0.1kHz'>
<img srcset='/images/green-1x1.png?f15 50w, /images/green-16x16.png?f15 51w' sizes='0.1dpi'>
<img srcset='/images/green-1x1.png?f16 50w, /images/green-16x16.png?f16 51w' sizes='0.1dpcm'>
<img srcset='/images/green-1x1.png?f17 50w, /images/green-16x16.png?f17 51w' sizes='0.1dppx'>
<img srcset='/images/green-1x1.png?f17a 50w, /images/green-16x16.png?f17a 51w' sizes='0.1x'>
<img srcset='/images/green-1x1.png?f18 50w, /images/green-16x16.png?f18 51w' data-foo='1px' sizes='attr(data-foo, length, 1px)'>
<img srcset='/images/green-1x1.png?f19 50w, /images/green-16x16.png?f19 51w' data-foo='1' sizes='attr(data-foo, px, 1px)'>
<img srcset='/images/green-1x1.png?f20 50w, /images/green-16x16.png?f20 51w' sizes='toggle(1px)'>
<img srcset='/images/green-1x1.png?f21 50w, /images/green-16x16.png?f21 51w' sizes='inherit'>
<img srcset='/images/green-1x1.png?f23 50w, /images/green-16x16.png?f23 51w' sizes='initial'>
<img srcset='/images/green-1x1.png?f24 50w, /images/green-16x16.png?f24 51w' sizes='unset'>
<img srcset='/images/green-1x1.png?f25 50w, /images/green-16x16.png?f25 51w' sizes='default'>
<img srcset='/images/green-1x1.png?f26 50w, /images/green-16x16.png?f26 51w' sizes='1/* */px'>
<img srcset='/images/green-1x1.png?f27 50w, /images/green-16x16.png?f27 51w' sizes='1p/* */x'>
<img srcset='/images/green-1x1.png?f28 50w, /images/green-16x16.png?f28 51w' sizes='-/**/0'>
<img srcset='/images/green-1x1.png?f29 50w, /images/green-16x16.png?f29 51w' sizes='((),1px'>
<img srcset='/images/green-1x1.png?f30 50w, /images/green-16x16.png?f30 51w' siz
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.bogus_comment",
      "message": "Bogus comment.",
      "severity": "Warning",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 27,
        "byte_start": 20,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 282,
        "byte_start": 191,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 360,
        "byte_start": 283,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 360,
        "byte_start": 283,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 456,
        "byte_start": 365,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 534,
        "byte_start": 457,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 534,
        "byte_start": 457,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 630,
        "byte_start": 539,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 708,
        "byte_start": 631,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 708,
        "byte_start": 631,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 804,
        "byte_start": 713,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 882,
        "byte_start": 805,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 882,
        "byte_start": 805,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 975,
        "byte_start": 888,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1061,
        "byte_start": 976,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1148,
        "byte_start": 1062,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1235,
        "byte_start": 1149,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1324,
        "byte_start": 1236,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1413,
        "byte_start": 1325,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1503,
        "byte_start": 1414,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1593,
        "byte_start": 1504,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1683,
        "byte_start": 1594,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1776,
        "byte_start": 1684,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1868,
        "byte_start": 1777,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1960,
        "byte_start": 1869,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2054,
        "byte_start": 1961,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2148,
        "byte_start": 2055,
        "col": 1,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2240,
        "byte_start": 2149,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2330,
        "byte_start": 2241,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2419,
        "byte_start": 2331,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2512,
        "byte_start": 2420,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2604,
        "byte_start": 2513,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2696,
        "byte_start": 2605,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2796,
        "byte_start": 2697,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2908,
        "byte_start": 2797,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(),1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3001,
        "byte_start": 2909,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3001,
        "byte_start": 2909,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “x(),1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3095,
        "byte_start": 3002,
        "col": 1,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3095,
        "byte_start": 3002,
        "col": 1,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “{},1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3188,
        "byte_start": 3096,
        "col": 1,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3188,
        "byte_start": 3096,
        "col": 1,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “[],1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3281,
        "byte_start": 3189,
        "col": 1,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3281,
        "byte_start": 3189,
        "col": 1,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px,(” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3373,
        "byte_start": 3282,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3373,
        "byte_start": 3282,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px,x(” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3466,
        "byte_start": 3374,
        "col": 1,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3466,
        "byte_start": 3374,
        "col": 1,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px,{” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3558,
        "byte_start": 3467,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3558,
        "byte_start": 3467,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px,[” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3650,
        "byte_start": 3559,
        "col": 1,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3650,
        "byte_start": 3559,
        "col": 1,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “\\(,1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3743,
        "byte_start": 3651,
        "col": 1,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3743,
        "byte_start": 3651,
        "col": 1,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “x\\(,1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3837,
        "byte_start": 3744,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3837,
        "byte_start": 3744,
        "col": 1,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “\\{,1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3930,
        "byte_start": 3838,
        "col": 1,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3930,
        "byte_start": 3838,
        "col": 1,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “\\[,1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4023,
        "byte_start": 3931,
        "col": 1,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4023,
        "byte_start": 3931,
        "col": 1,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1\\p\\x” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4115,
        "byte_start": 4024,
        "col": 1,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4115,
        "byte_start": 4024,
        "col": 1,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4211,
        "byte_start": 4116,
        "col": 1,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4315,
        "byte_start": 4212,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4420,
        "byte_start": 4316,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4530,
        "byte_start": 4421,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4648,
        "byte_start": 4531,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4767,
        "byte_start": 4649,
        "col": 1,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4877,
        "byte_start": 4768,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4988,
        "byte_start": 4878,
        "col": 1,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5120,
        "byte_start": 4989,
        "col": 1,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5224,
        "byte_start": 5121,
        "col": 1,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (min-width:0) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5339,
        "byte_start": 5225,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5339,
        "byte_start": 5225,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5465,
        "byte_start": 5340,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (min-width:unknown-mf-value) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5595,
        "byte_start": 5466,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5595,
        "byte_start": 5466,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5709,
        "byte_start": 5596,
        "col": 1,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (min-width:-1px) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5827,
        "byte_start": 5710,
        "col": 1,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5827,
        "byte_start": 5710,
        "col": 1,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5942,
        "byte_start": 5828,
        "col": 1,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (unknown-mf-name) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6061,
        "byte_start": 5943,
        "col": 1,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6061,
        "byte_start": 5943,
        "col": 1,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6187,
        "byte_start": 6062,
        "col": 1,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (unknown \"general-enclosed\") 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6317,
        "byte_start": 6188,
        "col": 1,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6317,
        "byte_start": 6188,
        "col": 1,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “unknown-general-enclosed(foo) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6444,
        "byte_start": 6318,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6444,
        "byte_start": 6318,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not unknown-general-enclosed(foo) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6575,
        "byte_start": 6445,
        "col": 1,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6575,
        "byte_start": 6445,
        "col": 1,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “print 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6678,
        "byte_start": 6576,
        "col": 1,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6678,
        "byte_start": 6576,
        "col": 1,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not print 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6785,
        "byte_start": 6679,
        "col": 1,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6785,
        "byte_start": 6679,
        "col": 1,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “unknown-media-type 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6901,
        "byte_start": 6786,
        "col": 1,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6901,
        "byte_start": 6786,
        "col": 1,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not unknown-media-type 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7021,
        "byte_start": 6902,
        "col": 1,
        "line": 78
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7021,
        "byte_start": 6902,
        "col": 1,
        "line": 78
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7142,
        "byte_start": 7022,
        "col": 1,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7267,
        "byte_start": 7143,
        "col": 1,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7403,
        "byte_start": 7268,
        "col": 1,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7527,
        "byte_start": 7404,
        "col": 1,
        "line": 82
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7663,
        "byte_start": 7528,
        "col": 1,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7800,
        "byte_start": 7664,
        "col": 1,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7918,
        "byte_start": 7801,
        "col": 1,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8051,
        "byte_start": 7919,
        "col": 1,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(123) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8154,
        "byte_start": 8052,
        "col": 1,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8154,
        "byte_start": 8052,
        "col": 1,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (123) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8261,
        "byte_start": 8155,
        "col": 1,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8261,
        "byte_start": 8155,
        "col": 1,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(!) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8362,
        "byte_start": 8262,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8362,
        "byte_start": 8262,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (!) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8467,
        "byte_start": 8363,
        "col": 1,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8467,
        "byte_start": 8363,
        "col": 1,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “! 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8566,
        "byte_start": 8468,
        "col": 1,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8566,
        "byte_start": 8468,
        "col": 1,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not ! 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8669,
        "byte_start": 8567,
        "col": 1,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8669,
        "byte_start": 8567,
        "col": 1,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(]) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8770,
        "byte_start": 8670,
        "col": 1,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8770,
        "byte_start": 8670,
        "col": 1,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (]) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8875,
        "byte_start": 8771,
        "col": 1,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8875,
        "byte_start": 8771,
        "col": 1,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “] 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8974,
        "byte_start": 8876,
        "col": 1,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8974,
        "byte_start": 8876,
        "col": 1,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not ] 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9077,
        "byte_start": 8975,
        "col": 1,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9077,
        "byte_start": 8975,
        "col": 1,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(}) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9178,
        "byte_start": 9078,
        "col": 1,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9178,
        "byte_start": 9078,
        "col": 1,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (}) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9283,
        "byte_start": 9179,
        "col": 1,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9283,
        "byte_start": 9179,
        "col": 1,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “} 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9382,
        "byte_start": 9284,
        "col": 1,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9382,
        "byte_start": 9284,
        "col": 1,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not } 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9485,
        "byte_start": 9383,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9485,
        "byte_start": 9383,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9584,
        "byte_start": 9486,
        "col": 1,
        "line": 101
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9584,
        "byte_start": 9486,
        "col": 1,
        "line": 101
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not ) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9687,
        "byte_start": 9585,
        "col": 1,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9687,
        "byte_start": 9585,
        "col": 1,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(;) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9788,
        "byte_start": 9688,
        "col": 1,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9788,
        "byte_start": 9688,
        "col": 1,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (;) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9893,
        "byte_start": 9789,
        "col": 1,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9893,
        "byte_start": 9789,
        "col": 1,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(.) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9994,
        "byte_start": 9894,
        "col": 1,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9994,
        "byte_start": 9894,
        "col": 1,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (.) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10099,
        "byte_start": 9995,
        "col": 1,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10099,
        "byte_start": 9995,
        "col": 1,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “; 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10198,
        "byte_start": 10100,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10198,
        "byte_start": 10100,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not ; 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10301,
        "byte_start": 10199,
        "col": 1,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10301,
        "byte_start": 10199,
        "col": 1,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10393,
        "byte_start": 10302,
        "col": 1,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10393,
        "byte_start": 10302,
        "col": 1,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px,” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10484,
        "byte_start": 10394,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10484,
        "byte_start": 10394,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) 1px,” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10589,
        "byte_start": 10485,
        "col": 1,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10589,
        "byte_start": 10485,
        "col": 1,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10683,
        "byte_start": 10590,
        "col": 1,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10781,
        "byte_start": 10684,
        "col": 1,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10875,
        "byte_start": 10782,
        "col": 1,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10969,
        "byte_start": 10876,
        "col": 1,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11062,
        "byte_start": 10970,
        "col": 1,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “all 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11163,
        "byte_start": 11063,
        "col": 1,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11163,
        "byte_start": 11063,
        "col": 1,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “all and (min-width:0) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11282,
        "byte_start": 11164,
        "col": 1,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11282,
        "byte_start": 11164,
        "col": 1,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “min-width:0 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11391,
        "byte_start": 11283,
        "col": 1,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11391,
        "byte_start": 11283,
        "col": 1,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px, 100vw” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11488,
        "byte_start": 11392,
        "col": 1,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11488,
        "byte_start": 11392,
        "col": 1,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px, (min-width:0) 100vw” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11599,
        "byte_start": 11489,
        "col": 1,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11599,
        "byte_start": 11489,
        "col": 1,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px, foo bar” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11700,
        "byte_start": 11600,
        "col": 1,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11700,
        "byte_start": 11600,
        "col": 1,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) 1px, foo bar” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11815,
        "byte_start": 11701,
        "col": 1,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11815,
        "byte_start": 11701,
        "col": 1,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11941,
        "byte_start": 11816,
        "col": 1,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (\"grammar does not match\") 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12071,
        "byte_start": 11942,
        "col": 1,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12071,
        "byte_start": 11942,
        "col": 1,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12199,
        "byte_start": 12072,
        "col": 1,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not (unknown-general-enclosed !) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12331,
        "byte_start": 12200,
        "col": 1,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12331,
        "byte_start": 12200,
        "col": 1,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12469,
        "byte_start": 12332,
        "col": 1,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not ((min-width:0) or (unknown \"general-enclosed\")) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12620,
        "byte_start": 12470,
        "col": 1,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12620,
        "byte_start": 12470,
        "col": 1,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12765,
        "byte_start": 12621,
        "col": 1,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “not ((max-width:0) or (unknown \"general-enclosed\")) 100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12916,
        "byte_start": 12766,
        "col": 1,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12916,
        "byte_start": 12766,
        "col": 1,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “calc(1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13011,
        "byte_start": 12917,
        "col": 1,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13011,
        "byte_start": 12917,
        "col": 1,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “min(1px, 200vw” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13114,
        "byte_start": 13012,
        "col": 1,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13114,
        "byte_start": 13012,
        "col": 1,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “max(-200vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13218,
        "byte_start": 13115,
        "col": 1,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13218,
        "byte_start": 13115,
        "col": 1,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) calc(1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13327,
        "byte_start": 13219,
        "col": 1,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13327,
        "byte_start": 13219,
        "col": 1,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) min(1px, 200vw” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13444,
        "byte_start": 13328,
        "col": 1,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13444,
        "byte_start": 13328,
        "col": 1,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) max(-200vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13562,
        "byte_start": 13445,
        "col": 1,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13562,
        "byte_start": 13445,
        "col": 1,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13657,
        "byte_start": 13568,
        "col": 1,
        "line": 140
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13742,
        "byte_start": 13658,
        "col": 1,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13742,
        "byte_start": 13658,
        "col": 1,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “,” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13828,
        "byte_start": 13743,
        "col": 1,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13828,
        "byte_start": 13743,
        "col": 1,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “-1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13917,
        "byte_start": 13829,
        "col": 1,
        "line": 143
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13917,
        "byte_start": 13829,
        "col": 1,
        "line": 143
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14003,
        "byte_start": 13918,
        "col": 1,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14003,
        "byte_start": 13918,
        "col": 1,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1%” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14092,
        "byte_start": 14004,
        "col": 1,
        "line": 145
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14092,
        "byte_start": 14004,
        "col": 1,
        "line": 145
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1deg” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14183,
        "byte_start": 14093,
        "col": 1,
        "line": 146
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14183,
        "byte_start": 14093,
        "col": 1,
        "line": 146
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1grad” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14275,
        "byte_start": 14184,
        "col": 1,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14275,
        "byte_start": 14184,
        "col": 1,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1rad” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14366,
        "byte_start": 14276,
        "col": 1,
        "line": 148
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14366,
        "byte_start": 14276,
        "col": 1,
        "line": 148
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1turn” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14460,
        "byte_start": 14367,
        "col": 1,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14460,
        "byte_start": 14367,
        "col": 1,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1s” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14551,
        "byte_start": 14461,
        "col": 1,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14551,
        "byte_start": 14461,
        "col": 1,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1ms” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14643,
        "byte_start": 14552,
        "col": 1,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14643,
        "byte_start": 14552,
        "col": 1,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1Hz” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14735,
        "byte_start": 14644,
        "col": 1,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14735,
        "byte_start": 14644,
        "col": 1,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1kHz” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14828,
        "byte_start": 14736,
        "col": 1,
        "line": 153
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14828,
        "byte_start": 14736,
        "col": 1,
        "line": 153
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1dpi” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14921,
        "byte_start": 14829,
        "col": 1,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14921,
        "byte_start": 14829,
        "col": 1,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1dpcm” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15015,
        "byte_start": 14922,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15015,
        "byte_start": 14922,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1dppx” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15109,
        "byte_start": 15016,
        "col": 1,
        "line": 156
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15109,
        "byte_start": 15016,
        "col": 1,
        "line": 156
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “0.1x” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15202,
        "byte_start": 15110,
        "col": 1,
        "line": 157
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15202,
        "byte_start": 15110,
        "col": 1,
        "line": 157
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “attr(data-foo, length, 1px)” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15331,
        "byte_start": 15203,
        "col": 1,
        "line": 158
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15331,
        "byte_start": 15203,
        "col": 1,
        "line": 158
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “attr(data-foo, px, 1px)” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15454,
        "byte_start": 15332,
        "col": 1,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15454,
        "byte_start": 15332,
        "col": 1,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “toggle(1px)” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15552,
        "byte_start": 15455,
        "col": 1,
        "line": 160
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15552,
        "byte_start": 15455,
        "col": 1,
        "line": 160
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “inherit” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15646,
        "byte_start": 15553,
        "col": 1,
        "line": 161
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15646,
        "byte_start": 15553,
        "col": 1,
        "line": 161
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “initial” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15740,
        "byte_start": 15647,
        "col": 1,
        "line": 162
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15740,
        "byte_start": 15647,
        "col": 1,
        "line": 162
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “unset” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15832,
        "byte_start": 15741,
        "col": 1,
        "line": 163
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15832,
        "byte_start": 15741,
        "col": 1,
        "line": 163
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “default” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15926,
        "byte_start": 15833,
        "col": 1,
        "line": 164
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15926,
        "byte_start": 15833,
        "col": 1,
        "line": 164
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1/* */px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16021,
        "byte_start": 15927,
        "col": 1,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16021,
        "byte_start": 15927,
        "col": 1,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1p/* */x” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16116,
        "byte_start": 16022,
        "col": 1,
        "line": 166
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16116,
        "byte_start": 16022,
        "col": 1,
        "line": 166
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “-/**/0” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16209,
        "byte_start": 16117,
        "col": 1,
        "line": 167
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16209,
        "byte_start": 16117,
        "col": 1,
        "line": 167
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “((),1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16303,
        "byte_start": 16210,
        "col": 1,
        "line": 168
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16303,
        "byte_start": 16210,
        "col": 1,
        "line": 168
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “x(x(),1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16399,
        "byte_start": 16304,
        "col": 1,
        "line": 169
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16399,
        "byte_start": 16304,
        "col": 1,
        "line": 169
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “{{},1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16493,
        "byte_start": 16400,
        "col": 1,
        "line": 170
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16493,
        "byte_start": 16400,
        "col": 1,
        "line": 170
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “[[],1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16587,
        "byte_start": 16494,
        "col": 1,
        "line": 171
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16587,
        "byte_start": 16494,
        "col": 1,
        "line": 171
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1px !important” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16688,
        "byte_start": 16588,
        "col": 1,
        "line": 172
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16688,
        "byte_start": 16588,
        "col": 1,
        "line": 172
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “\\1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16779,
        "byte_start": 16689,
        "col": 1,
        "line": 173
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16779,
        "byte_start": 16689,
        "col": 1,
        "line": 173
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “all 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16873,
        "byte_start": 16780,
        "col": 1,
        "line": 174
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16873,
        "byte_start": 16780,
        "col": 1,
        "line": 174
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “all and (min-width:0) 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 16985,
        "byte_start": 16874,
        "col": 1,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 16985,
        "byte_start": 16874,
        "col": 1,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “min-width:0 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17087,
        "byte_start": 16986,
        "col": 1,
        "line": 176
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17087,
        "byte_start": 16986,
        "col": 1,
        "line": 176
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “100vw, 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17184,
        "byte_start": 17088,
        "col": 1,
        "line": 177
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17184,
        "byte_start": 17088,
        "col": 1,
        "line": 177
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “100vw, (min-width:0) 1px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17295,
        "byte_start": 17185,
        "col": 1,
        "line": 178
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17295,
        "byte_start": 17185,
        "col": 1,
        "line": 178
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “foo bar” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17389,
        "byte_start": 17296,
        "col": 1,
        "line": 179
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17389,
        "byte_start": 17296,
        "col": 1,
        "line": 179
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “foo-bar” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17483,
        "byte_start": 17390,
        "col": 1,
        "line": 180
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17483,
        "byte_start": 17390,
        "col": 1,
        "line": 180
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) 1px foo bar” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17595,
        "byte_start": 17484,
        "col": 1,
        "line": 181
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17595,
        "byte_start": 17484,
        "col": 1,
        "line": 181
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) 0.1%” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17700,
        "byte_start": 17596,
        "col": 1,
        "line": 182
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17700,
        "byte_start": 17596,
        "col": 1,
        "line": 182
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “(min-width:0) 1” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17802,
        "byte_start": 17701,
        "col": 1,
        "line": 183
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17802,
        "byte_start": 17701,
        "col": 1,
        "line": 183
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “-1e0px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17895,
        "byte_start": 17803,
        "col": 1,
        "line": 184
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17895,
        "byte_start": 17803,
        "col": 1,
        "line": 184
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “1e1.5px” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 17989,
        "byte_start": 17896,
        "col": 1,
        "line": 185
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 17989,
        "byte_start": 17896,
        "col": 1,
        "line": 185
      }
    },
    {
      "category": "Html",
      "code": "html.img.sizes.invalid",
      "message": "Bad value “var(--foo)” for attribute “sizes” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 18105,
        "byte_start": 17990,
        "col": 1,
        "line": 186
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 18105,
        "byte_start": 17990,
        "col": 1,
        "line": 186
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/sizes/support/sizes-iframed.sub.html"
}
```
