# html/semantics/embedded-content/the-img-element/srcset/parse-a-srcset-attribute.html

Counts:
- errors: 0
- warnings: 626
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/srcset/parse-a-srcset-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img parse a srcset attribute</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src=common.js></script>
<div id=log></div>
<!-- splitting loop -->
<img srcset='' data-expect=''>
<img srcset=',' data-expect=''>
<img srcset=',,,' data-expect=''>
<img srcset='  data:,a  1x  ' data-expect='data:,a'>
<img srcset='&#x9;&#x9;data:,a&#x9;&#x9;1x&#x9;&#x9;' data-expect='data:,a'>
<img srcset='&#xA;&#xA;data:,a&#xA;&#xA;1x&#xA;&#xA;' data-expect='data:,a'>
<img srcset='&#xB;&#xB;data:,a&#xB;&#xB;1x&#xB;&#xB;' data-expect='&#xB;&#xB;data:,a&#xB;&#xB;1x&#xB;&#xB;' data-resolve>
<img srcset='&#xC;&#xC;data:,a&#xC;&#xC;1x&#xC;&#xC;' data-expect='data:,a'>
<img srcset='&#xD;&#xD;data:,a&#xD;&#xD;1x&#xD;&#xD;' data-expect='data:,a'>
<img srcset='&#xE;&#xE;data:,a&#xE;&#xE;1x&#xE;&#xE;' data-expect='&#xE;&#xE;data:,a&#xE;&#xE;1x&#xE;&#xE;' data-resolve>
<img srcset='&#xF;&#xF;data:,a&#xF;&#xF;1x&#xF;&#xF;' data-expect='&#xF;&#xF;data:,a&#xF;&#xF;1x&#xF;&#xF;' data-resolve>
<img srcset='&#x10;&#x10;data:,a&#x10;&#x10;1x&#x10;&#x10;' data-expect='&#x10;&#x10;data:,a&#x10;&#x10;1x&#x10;&#x10;' data-resolve>
<img srcset='data:,a' data-expect='data:,a'>
<img srcset='data:,a ' data-expect='data:,a'>
<img srcset='data:,a ,' data-expect='data:,a'>
<img srcset='data:,a,' data-expect='data:,a'>
<img srcset='data:,a, ' data-expect='data:,a'>
<img srcset='data:,a,,,' data-expect='data:,a'>
<img srcset='data:,a,, , ' data-expect='data:,a'>
<img srcset=' data:,a' data-expect='data:,a'>
<img srcset=',,,data:,a' data-expect='data:,a'>
<img srcset=' , ,,data:,a' data-expect='data:,a'>
<img srcset='&nbsp;data:,a' data-expect='&nbsp;data:,a' data-resolve>
<img srcset='data:,a&nbsp;' data-expect='data:,a&nbsp;' data-resolve>
<!-- descriptor tokenizer -->
<img srcset='data:,a 1x' data-expect='data:,a'>
<img srcset='data:,a 1x ' data-expect='data:,a'>
<img srcset='data:,a 1x,' data-expect='data:,a'>
<img srcset='data:,a ( , data:,b 1x, ), data:,c' data-expect='data:,c'>
<img srcset='data:,a ((( , data:,b 1x, ), data:,c' data-expect='data:,c'>
<img srcset='data:,a [ , data:,b 1x, ], data:,c' data-expect='data:,b'>
<img srcset='data:,a { , data:,b 1x, }, data:,c' data-expect='data:,b'>
<img srcset='data:,a " , data:,b 1x, ", data:,c' data-expect='data:,b'>
<img srcset='data:,a \,data:;\,b, data:,c' data-expect='data:;\,b'>
<img srcset='data:,a, data:,b (' data-expect='data:,a'>
<img srcset='data:,a, data:,b (  ' data-expect='data:,a'>
<img srcset='data:,a, data:,b (,' data-expect='data:,a'>
<img srcset='data:,a, data:,b (x' data-expect='data:,a'>
<img srcset='data:,a, data:,b ()' data-expect='data:,a'>
<img srcset='data:,a (, data:,b' data-expect=''>
<img srcset='data:,a /*, data:,b, data:,c */' data-expect='data:,b'>
<img srcset='data:,a //, data:,b' data-expect='data:,b'>
<!-- descriptor parser -->
<img srcset='data:,a foo' data-expect=''>
<img srcset='data:,a foo foo' data-expect=''>
<img srcset='data:,a foo 1x' data-expect=''>
<img srcset='data:,a foo 1x foo' data-expect=''>
<img srcset='data:,a foo 1w' data-expect=''>
<img srcset='data:,a foo 1w foo' data-expect=''>
<img srcset='data:,a 1x 1x' data-expect=''>
<img srcset='data:,a 1w 1w' data-expect=''>
<img srcset='data:,a 1w 1x' data-expect=''>
<img srcset='data:,a 1x 1w' data-expect=''>
<img srcset='data:,a 1w 1h' data-expect='data:,a'><!-- should fail for x-only impl -->
<img srcset='data:,a 1h 1w' data-expect='data:,a'><!-- should fail for x-only impl -->
<img srcset='data:,a 1h 1h' data-expect=''>
<img srcset='data:,a 1h 1x' data-expect=''>
<img srcset='data:,a 1h 1w 1x' data-expect=''>
<img srcset='data:,a 1x 1w 1h' data-expect=''>
<img srcset='data:,a 1w' data-expect='data:,a'><!-- should fail for x-only impl -->
<img srcset='data:,a 1h' data-expect=''>
<img srcset='data:,a 1h foo' data-expect=''>
<img srcset='data:,a foo 1h' data-expect=''>
<img srcset='data:,a 0w' data-expect=''>
<img srcset='data:,a -1w' data-expect=''>
<img srcset='data:,a 1w -1w' data-expect=''>
<img srcset='data:,a 1.0w' data-expect=''>
<img srcset='data:,a 1w 1.0w' data-expect=''>
<img srcset='data:,a 1e0w' data-expect=''>
<img srcset='data:,a 1w 1e0w' data-expect=''>
<img srcset='data:,a 1www' data-expect=''>
<img srcset='data:,a 1w 1www' data-expect=''>
<img srcset='data:,a +1w' data-expect=''>
<img srcset='data:,a 1w +1w' data-expect=''>
<img srcset='data:,a 1W' data-expect=''>
<img srcset='data:,a 1w 1W' data-expect=''>
<img srcset='data:,a Infinityw' data-expect=''>
<img srcset='data:,a 1w Infinityw' data-expect=''>
<img srcset='data:,a NaNw' data-expect=''>
<img srcset='data:,a 1w NaNw' data-expect=''>
<img srcset='data:,a 0x1w' data-expect=''>
<img srcset='data:,a 0X1w' data-expect=''>
<img srcset='data:,a 1&#x1;w' data-expect='' data-desc='trailing U+0001'>
<img srcset='data:,a 1&nbsp;w' data-expect='' data-desc='trailing U+00A0'>
<img srcset='data:,a 1&#x1680;w' data-expect='' data-desc='trailing U+1680'>
<img srcset='data:,a 1&#x2000;w' data-expect='' data-desc='trailing U+2000'>
<img srcset='data:,a 1&#x2001;w' data-expect='' data-desc='trailing U+2001'>
<img srcset='data:,a 1&#x2002;w' data-expect='' data-desc='trailing U+2002'>
<img srcset='data:,a 1&#x2003;w' data-expect='' data-desc='trailing U+2003'>
<img srcset='data:,a 1&#x2004;w' data-expect='' data-desc='trailing U+2004'>
<img srcset='data:,a 1&#x2005;w' data-expect='' data-desc='trailing U+2005'>
<img srcset='data:,a 1&#x2006;w' data-expect='' data-desc='trailing U+2006'>
<img srcset='data:,a 1&#x2007;w' data-expect='' data-desc='trailing U+2007'>
<img srcset='data:,a 1&#x2008;w' data-expect='' data-desc='trailing U+2008'>
<img srcset='data:,a 1&#x2009;w' data-expect='' data-desc='trailing U+2009'>
<img srcset='data:,a 1&#x200A;w' data-expect='' data-desc='trailing U+200A'>
<img srcset='data:,a 1&#x200C;w' data-expect='' data-desc='trailing U+200C'>
<img srcset='data:,a 1&#x200D;w' data-expect='' data-desc='trailing U+200D'>
<img srcset='data:,a 1&#x202F;w' data-expect='' data-desc='trailing U+202F'>
<img srcset='data:,a 1&#x205F;w' data-expect='' data-desc='trailing U+205F'>
<img srcset='data:,a 1&#x3000;w' data-expect='' data-desc='trailing U+3000'>
<img srcset='data:,a 1&#xFEFF;w' data-expect='' data-desc='trailing U+FEFF'>
<img srcset='data:,a &#x1;1w' data-expect='' data-desc='leading U+0001'>
<img srcset='data:,a &nbsp;1w' data-expect='' data-desc='leading U+00A0'>
<img srcset='data:,a &#x1680;1w' data-expect='' data-desc='leading U+1680'>
<img srcset='data:,a &#x2000;1w' data-expect='' data-desc='leading U+2000'>
<img srcset='data:,a &#x2001;1w' data-expect='' data-desc='leading U+2001'>
<img srcset='data:,a &#x2002;1w' data-expect='' data-desc='leading U+2002'>
<img srcset='data:,a &#x2003;1w' data-expect='' data-desc='leading U+2003'>
<img srcset='data:,a &#x2004;1w' data-expect='' data-desc='leading U+2004'>
<img srcset='data:,a &#x2005;1w' data-expect='' data-desc='leading U+2005'>
<img srcset='data:,a &#x2006;1w' data-expect='' data-desc='leading U+2006'>
<img srcset='data:,a &#x2007;1w' data-expect='' data-desc='leading U+2007'>
<img srcset='data:,a &#x2008;1w' data-expect='' data-desc='leading U+2008'>
<img srcset='data:,a &#x2009;1w' data-expect='' data-desc='leading U+2009'>
<img srcset='data:,a &#x200A;1w' data-expect='' data-desc='leading U+200A'>
<img srcset='data:,a &#x200C;1w' data-expect='' data-desc='leading U+200C'>
<img srcset='data:,a &#x200D;1w' data-expect='' data-desc='leading U+200D'>
<img srcset='data:,a &#x202F;1w' data-expect='' data-desc='leading U+202F'>
<img srcset='data:,a &#x205F;1w' data-expect='' data-desc='leading U+205F'>
<img srcset='data:,a &#x3000;1w' data-expect='' data-desc='leading U+3000'>
<img srcset='data:,a &#xFEFF;1w' data-expect='' data-desc='leading U+FEFF'>
<img srcset='data:,a 0x' data-expect='data:,a'>
<img srcset='data:,a -0x' data-expect='data:,a'>
<img srcset='data:,a 1x -0x' data-expect=''>
<img srcset='data:,a -1x' data-expect=''>
<img srcset='data:,a 1x -1x' data-expect=''>
<img srcset='data:,a 1e0x' data-expect='data:,a'>
<img srcset='data:,a 1E0x' data-expect='data:,a'>
<img srcset='data:,a 1e-1x' data-expect='data:,a'>
<img srcset='data:,a 1.5e1x' data-expect='data:,a'>
<img srcset='data:,a -x' data-expect=''>
<img srcset='data:,a .x' data-expect=''>
<img srcset='data:,a -.x' data-expect=''>
<img srcset='data:,a 1.x' data-expect=''>
<img srcset='data:,a .5x' data-expect='data:,a'>
<img srcset='data:,a .5e1x' data-expect='data:,a'>
<img srcset='data:,a 1x 1.5e1x' data-expect=''>
<img srcset='data:,a 1x 1e1.5x' data-expect=''>
<img srcset='data:,a 1.0x' data-expect='data:,a'>
<img srcset='data:,a 1x 1.0x' data-expect=''>
<img srcset='data:,a +1x' data-expect=''>
<img srcset='data:,a 1X' data-expect=''>
<img srcset='data:,a Infinityx' data-expect=''>
<img srcset='data:,a NaNx' data-expect=''>
<img srcset='data:,a 0x1x' data-expect=''>
<img srcset='data:,a 0X1x' data-expect=''>
<img srcset='data:,a 1&#x1;x' data-expect='' data-desc='trailing U+0001'>
<img srcset='data:,a 1&nbsp;x' data-expect='' data-desc='trailing U+00A0'>
<img srcset='data:,a 1&#x1680;x' data-expect='' data-desc='trailing U+1680'>
<img srcset='data:,a 1&#x2000;x' data-expect='' data-desc='trailing U+2000'>
<img srcset='data:,a 1&#x2001;x' data-expect='' data-desc='trailing U+2001'>
<img srcset='data:,a 1&#x2002;x' data-expect='' data-desc='trailing U+2002'>
<img srcset='data:,a 1&#x2003;x' data-expect='' data-desc='trailing U+2003'>
<img srcset='data:,a 1&#x2004;x' data-expect='' data-desc='trailing U+2004'>
<img srcset='data:,a 1&#x2005;x' data-expect='' data-desc='trailing U+2005'>
<img srcset='data:,a 1&#x2006;x' data-expect='' data-desc='trailing U+2006'>
<img srcset='data:,a 1&#x2007;x' data-expect='' data-desc='trailing U+2007'>
<img srcset='data:,a 1&#x2008;x' data-expect='' data-desc='trailing U+2008'>
<img srcset='data:,a 1&#x2009;x' data-expect='' data-desc='trailing U+2009'>
<img srcset='data:,a 1&#x200A;x' data-expect='' data-desc='trailing U+200A'>
<img srcset='data:,a 1&#x200C;x' data-expect='' data-desc='trailing U+200C'>
<img srcset='data:,a 1&#x200D;x' data-expect='' data-desc='trailing U+200D'>
<img srcset='data:,a 1&#x202F;x' data-expect='' data-desc='trailing U+202F'>
<img srcset='data:,a 1&#x205F;x' data-expect='' data-desc='trailing U+205F'>
<img srcset='data:,a 1&#x3000;x' data-expect='' data-desc='trailing U+3000'>
<img srcset='data:,a 1&#xFEFF;x' data-expect='' data-desc='trailing U+FEFF'>
<img srcset='data:,a &#x1;1x' data-expect='' data-desc='leading U+0001'>
<img srcset='data:,a &nbsp;1x' data-expect='' data-desc='leading U+00A0'>
<img srcset='data:,a &#x1680;1x' data-expect='' data-desc='leading U+1680'>
<img srcset='data:,a &#x2000;1x' data-expect='' data-desc='leading U+2000'>
<img srcset='data:,a &#x2001;1x' data-expect='' data-desc='leading U+2001'>
<img srcset='data:,a &#x2002;1x' data-expect='' data-desc='leading U+2002'>
<img srcset='data:,a &#x2003;1x' data-expect='' data-desc='leading U+2003'>
<img srcset='data:,a &#x2004;1x' data-expect='' data-desc='leading U+2004'>
<img srcset='data:,a &#x2005;1x' data-expect='' data-desc='leading U+2005'>
<img srcset='data:,a &#x2006;1x' data-expect='' data-desc='leading U+2006'>
<img srcset='data:,a &#x2007;1x' data-expect='' data-desc='leading U+2007'>
<img srcset='data:,a &#x2008;1x' data-expect='' data-desc='leading U+2008'>
<img srcset='data:,a &#x2009;1x' data-expect='' data-desc='leading U+2009'>
<img srcset='data:,a &#x200A;1x' data-expect='' data-desc='leading U+200A'>
<img srcset='data:,a &#x200C;1x' data-expect='' data-desc='leading U+200C'>
<img srcset='data:,a &#x200D;1x' data-expect='' data-desc='leading U+200D'>
<img srcset='data:,a &#x202F;1x' data-expect='' data-desc='leading U+202F'>
<img srcset='data:,a &#x205F;1x' data-expect='' data-desc='leading U+205F'>
<img srcset='data:,a &#x3000;1x' data-expect='' data-desc='leading U+3000'>
<img srcset='data:,a &#xFEFF;1x' data-expect='' data-desc='leading U+FEFF'>
<img srcset='data:,a 1w 0h' data-expect=''>
<img srcset='data:,a 1w -1h' data-expect=''>
<img srcset='data:,a 1w 1.0h' data-expect=''>
<img srcset='data:,a 1w 1e0h' data-expect=''>
<img srcset='data:,a 1w 1hhh' data-expect=''>
<img srcset='data:,a 1w +1h' data-expect=''>
<img srcset='data:,a 1w 1H' data-expect=''>
<img srcset='data:,a 1w Infinityh' data-expect=''>
<img srcset='data:,a 1w NaNh' data-expect=''>
<img srcset='data:,a 0x1h' data-expect=''>
<img srcset='data:,a 0X1h' data-expect=''>
<img srcset='data:,a 1w 1&#x1;h' data-expect='' data-desc='trailing U+0001'>
<img srcset='data:,a 1w 1&nbsp;h' data-expect='' data-desc='trailing U+00A0'>
<img srcset='data:,a 1w 1&#x1680;h' data-expect='' data-desc='trailing U+1680'>
<img srcset='data:,a 1w 1&#x2000;h' data-expect='' data-desc='trailing U+2000'>
<img srcset='data:,a 1w 1&#x2001;h' data-expect='' data-desc='trailing U+2001'>
<img srcset='data:,a 1w 1&#x2002;h' data-expect='' data-desc='trailing U+2002'>
<img srcset='data:,a 1w 1&#x2003;h' data-expect='' data-desc='trailing U+2003'>
<img srcset='data:,a 1w 1&#x2004;h' data-expect='' data-desc='trailing U+2004'>
<img srcset='data:,a 1w 1&#x2005;h' data-expect='' data-desc='trailing U+2005'>
<img srcset='data:,a 1w 1&#x2006;h' data-expect='' data-desc='trailing U+2006'>
<img srcset='data:,a 1w 1&#x2007;h' data-expect='' data-desc='trailing U+2007'>
<img srcset='data:,a 1w 1&#x2008;h' data-expect='' data-desc='trailing U+2008'>
<img srcset='data:,a 1w 1&#x2009;h' data-expect='' data-desc='trailing U+2009'>
<img srcset='data:,a 1w 1&#x200A;h' data-expect='' data-desc='trailing U+200A'>
<img srcset='data:,a 1w 1&#x200C;h' data-expect='' data-desc='trailing U+200C'>
<img srcset='data:,a 1w 1&#x200D;h' data-expect='' data-desc='trailing U+200D'>
<img srcset='data:,a 1w 1&#x202F;h' data-expect='' data-desc='trailing U+202F'>
<img srcset='data:,a 1w 1&#x205F;h' data-expect='' data-desc='trailing U+205F'>
<img srcset='data:,a 1w 1&#x3000;h' data-expect='' data-desc='trailing U+3000'>
<img srcset='data:,a 1w 1&#xFEFF;h' data-expect='' data-desc='trailing U+FEFF'>
<img srcset='data:,a 1w &#x1;1h' data-expect='' data-desc='leading U+0001'>
<img srcset='data:,a 1w &nbsp;1h' data-expect='' data-desc='leading U+00A0'>
<img srcset='data:,a 1w &#x1680;1h' data-expect='' data-desc='leading U+1680'>
<img srcset='data:,a 1w &#x2000;1h' data-expect='' data-desc='leading U+2000'>
<img srcset='data:,a 1w &#x2001;1h' data-expect='' data-desc='leading U+2001'>
<img srcset='data:,a 1w &#x2002;1h' data-expect='' data-desc='leading U+2002'>
<img srcset='data:,a 1w &#x2003;1h' data-expect='' data-desc='leading U+2003'>
<img srcset='data:,a 1w &#x2004;1h' data-expect='' data-desc='leading U+2004'>
<img srcset='data:,a 1w &#x2005;1h' data-expect='' data-desc='leading U+2005'>
<img srcset='data:,a 1w &#x2006;1h' data-expect='' data-desc='leading U+2006'>
<img srcset='data:,a 1w &#x2007;1h' data-expect='' data-desc='leading U+2007'>
<img srcset='data:,a 1w &#x2008;1h' data-expect='' data-desc='leading U+2008'>
<img srcset='data:,a 1w &#x2009;1h' data-expect='' data-desc='leading U+2009'>
<img srcset='data:,a 1w &#x200A;1h' data-expect='' data-desc='leading U+200A'>
<img srcset='data:,a 1w &#x200C;1h' data-expect='' data-desc='leading U+200C'>
<img srcset='data:,a 1w &#x200D;1h' data-expect='' data-desc='leading U+200D'>
<img srcset='data:,a 1w &#x202F;1h' data-expect='' data-desc='leading U+202F'>
<img srcset='data:,a 1w &#x205F;1h' data-expect='' data-desc='leading U+205F'>
<img srcset='data:,a 1w &#x3000;1h' data-expect='' data-desc='leading U+3000'>
<img srcset='data:,a 1w &#xFEFF;1h' data-expect='' data-desc='leading U+FEFF'>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 271,
        "byte_start": 241,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 271,
        "byte_start": 241,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 303,
        "byte_start": 272,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 303,
        "byte_start": 272,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “,,,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 337,
        "byte_start": 304,
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
        "byte_end": 337,
        "byte_start": 304,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “  data:,a  1x  ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 390,
        "byte_start": 338,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 390,
        "byte_start": 338,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “\t\tdata:,a\t\t1x\t\t” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 467,
        "byte_start": 391,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 467,
        "byte_start": 391,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “\n\ndata:,a\n\n1x\n\n” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 544,
        "byte_start": 468,
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
        "byte_end": 544,
        "byte_start": 468,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 546,
        "byte_start": 545,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 546,
        "byte_start": 545,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “\u000b\u000bdata:,a\u000b\u000b1x\u000b\u000b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 666,
        "byte_start": 545,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 666,
        "byte_start": 545,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 551,
        "byte_start": 550,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 551,
        "byte_start": 550,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 563,
        "byte_start": 562,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 563,
        "byte_start": 562,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 568,
        "byte_start": 567,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 568,
        "byte_start": 567,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 575,
        "byte_start": 574,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 575,
        "byte_start": 574,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 580,
        "byte_start": 579,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000b).",
      "severity": "Warning",
      "span": {
        "byte_end": 580,
        "byte_start": 579,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “\f\fdata:,a\f\f1x\f\f” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 743,
        "byte_start": 667,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 743,
        "byte_start": 667,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 745,
        "byte_start": 744,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “\r\rdata:,a\r\r1x\r\r” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 820,
        "byte_start": 744,
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
        "byte_end": 820,
        "byte_start": 744,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 750,
        "byte_start": 749,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 762,
        "byte_start": 761,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 767,
        "byte_start": 766,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 774,
        "byte_start": 773,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 779,
        "byte_start": 778,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 822,
        "byte_start": 821,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 822,
        "byte_start": 821,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 942,
        "byte_start": 821,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 827,
        "byte_start": 826,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 827,
        "byte_start": 826,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 839,
        "byte_start": 838,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 839,
        "byte_start": 838,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 844,
        "byte_start": 843,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 844,
        "byte_start": 843,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 851,
        "byte_start": 850,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 851,
        "byte_start": 850,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 855,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000e).",
      "severity": "Warning",
      "span": {
        "byte_end": 856,
        "byte_start": 855,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 944,
        "byte_start": 943,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 944,
        "byte_start": 943,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1064,
        "byte_start": 943,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 948,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 949,
        "byte_start": 948,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 961,
        "byte_start": 960,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 961,
        "byte_start": 960,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 966,
        "byte_start": 965,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 966,
        "byte_start": 965,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 973,
        "byte_start": 972,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 973,
        "byte_start": 972,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 978,
        "byte_start": 977,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+000f).",
      "severity": "Warning",
      "span": {
        "byte_end": 978,
        "byte_start": 977,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1066,
        "byte_start": 1065,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1066,
        "byte_start": 1065,
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
        "byte_end": 1198,
        "byte_start": 1065,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1072,
        "byte_start": 1071,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1072,
        "byte_start": 1071,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1085,
        "byte_start": 1084,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1085,
        "byte_start": 1084,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1091,
        "byte_start": 1090,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1091,
        "byte_start": 1090,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1099,
        "byte_start": 1098,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1099,
        "byte_start": 1098,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1105,
        "byte_start": 1104,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0010).",
      "severity": "Warning",
      "span": {
        "byte_end": 1105,
        "byte_start": 1104,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1243,
        "byte_start": 1199,
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
        "byte_end": 1243,
        "byte_start": 1199,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1289,
        "byte_start": 1244,
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
        "byte_end": 1289,
        "byte_start": 1244,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1336,
        "byte_start": 1290,
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
        "byte_end": 1336,
        "byte_start": 1290,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1382,
        "byte_start": 1337,
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
        "byte_end": 1382,
        "byte_start": 1337,
        "col": 1,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a, ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1429,
        "byte_start": 1383,
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
        "byte_end": 1429,
        "byte_start": 1383,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a,,,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1477,
        "byte_start": 1430,
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
        "byte_end": 1477,
        "byte_start": 1430,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a,, , ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1527,
        "byte_start": 1478,
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
        "byte_end": 1527,
        "byte_start": 1478,
        "col": 1,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “ data:,a” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1573,
        "byte_start": 1528,
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
        "byte_end": 1573,
        "byte_start": 1528,
        "col": 1,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “,,,data:,a” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1621,
        "byte_start": 1574,
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
        "byte_end": 1621,
        "byte_start": 1574,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “ , ,,data:,a” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1671,
        "byte_start": 1622,
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
        "byte_end": 1671,
        "byte_start": 1622,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “ data:,a” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1741,
        "byte_start": 1672,
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
        "byte_end": 1741,
        "byte_start": 1672,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1811,
        "byte_start": 1742,
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
        "byte_end": 1811,
        "byte_start": 1742,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1889,
        "byte_start": 1842,
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
        "byte_end": 1889,
        "byte_start": 1842,
        "col": 1,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1938,
        "byte_start": 1890,
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
        "byte_end": 1938,
        "byte_start": 1890,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1987,
        "byte_start": 1939,
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
        "byte_end": 1987,
        "byte_start": 1939,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ( , data:,b 1x, ), data:,c” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2059,
        "byte_start": 1988,
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
        "byte_end": 2059,
        "byte_start": 1988,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ((( , data:,b 1x, ), data:,c” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2133,
        "byte_start": 2060,
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
        "byte_end": 2133,
        "byte_start": 2060,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a [ , data:,b 1x, ], data:,c” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2205,
        "byte_start": 2134,
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
        "byte_end": 2205,
        "byte_start": 2134,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a { , data:,b 1x, }, data:,c” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2277,
        "byte_start": 2206,
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
        "byte_end": 2277,
        "byte_start": 2206,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a \" , data:,b 1x, \", data:,c” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2349,
        "byte_start": 2278,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2349,
        "byte_start": 2278,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a \\,data:;\\,b, data:,c” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2417,
        "byte_start": 2350,
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
        "byte_end": 2417,
        "byte_start": 2350,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a, data:,b (” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2473,
        "byte_start": 2418,
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
        "byte_end": 2473,
        "byte_start": 2418,
        "col": 1,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a, data:,b (  ” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2531,
        "byte_start": 2474,
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
        "byte_end": 2531,
        "byte_start": 2474,
        "col": 1,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a, data:,b (,” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2588,
        "byte_start": 2532,
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
        "byte_end": 2588,
        "byte_start": 2532,
        "col": 1,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a, data:,b (x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2645,
        "byte_start": 2589,
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
        "byte_end": 2645,
        "byte_start": 2589,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a, data:,b ()” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2702,
        "byte_start": 2646,
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
        "byte_end": 2702,
        "byte_start": 2646,
        "col": 1,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a (, data:,b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2751,
        "byte_start": 2703,
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
        "byte_end": 2751,
        "byte_start": 2703,
        "col": 1,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a /*, data:,b, data:,c */” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2820,
        "byte_start": 2752,
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
        "byte_end": 2820,
        "byte_start": 2752,
        "col": 1,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a //, data:,b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2877,
        "byte_start": 2821,
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
        "byte_end": 2877,
        "byte_start": 2821,
        "col": 1,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2946,
        "byte_start": 2905,
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
        "byte_end": 2946,
        "byte_start": 2905,
        "col": 1,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo foo” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2992,
        "byte_start": 2947,
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
        "byte_end": 2992,
        "byte_start": 2947,
        "col": 1,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3037,
        "byte_start": 2993,
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
        "byte_end": 3037,
        "byte_start": 2993,
        "col": 1,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo 1x foo” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3086,
        "byte_start": 3038,
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
        "byte_end": 3086,
        "byte_start": 3038,
        "col": 1,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3131,
        "byte_start": 3087,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3131,
        "byte_start": 3087,
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
        "byte_end": 3131,
        "byte_start": 3087,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo 1w foo” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3180,
        "byte_start": 3132,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3180,
        "byte_start": 3132,
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
        "byte_end": 3180,
        "byte_start": 3132,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3224,
        "byte_start": 3181,
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
        "byte_end": 3224,
        "byte_start": 3181,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3268,
        "byte_start": 3225,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3268,
        "byte_start": 3225,
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
        "byte_end": 3268,
        "byte_start": 3225,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3312,
        "byte_start": 3269,
        "col": 1,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3312,
        "byte_start": 3269,
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
        "byte_end": 3312,
        "byte_start": 3269,
        "col": 1,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3356,
        "byte_start": 3313,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3356,
        "byte_start": 3313,
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
        "byte_end": 3356,
        "byte_start": 3313,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3407,
        "byte_start": 3357,
        "col": 1,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3407,
        "byte_start": 3357,
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
        "byte_end": 3407,
        "byte_start": 3357,
        "col": 1,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1h 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3494,
        "byte_start": 3444,
        "col": 1,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3494,
        "byte_start": 3444,
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
        "byte_end": 3494,
        "byte_start": 3444,
        "col": 1,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1h 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3574,
        "byte_start": 3531,
        "col": 1,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3574,
        "byte_start": 3531,
        "col": 1,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1h 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3618,
        "byte_start": 3575,
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
        "byte_end": 3618,
        "byte_start": 3575,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1h 1w 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3665,
        "byte_start": 3619,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3665,
        "byte_start": 3619,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3665,
        "byte_start": 3619,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x 1w 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3712,
        "byte_start": 3666,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3712,
        "byte_start": 3666,
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
        "byte_end": 3712,
        "byte_start": 3666,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3760,
        "byte_start": 3713,
        "col": 1,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3760,
        "byte_start": 3713,
        "col": 1,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3760,
        "byte_start": 3713,
        "col": 1,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3837,
        "byte_start": 3797,
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
        "byte_end": 3837,
        "byte_start": 3797,
        "col": 1,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1h foo” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3882,
        "byte_start": 3838,
        "col": 1,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3882,
        "byte_start": 3838,
        "col": 1,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a foo 1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3927,
        "byte_start": 3883,
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
        "byte_end": 3927,
        "byte_start": 3883,
        "col": 1,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3968,
        "byte_start": 3928,
        "col": 1,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 3968,
        "byte_start": 3928,
        "col": 1,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3968,
        "byte_start": 3928,
        "col": 1,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a -1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4010,
        "byte_start": 3969,
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
        "byte_end": 4010,
        "byte_start": 3969,
        "col": 1,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w -1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4055,
        "byte_start": 4011,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4055,
        "byte_start": 4011,
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
        "byte_end": 4055,
        "byte_start": 4011,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1.0w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4098,
        "byte_start": 4056,
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
        "byte_end": 4098,
        "byte_start": 4056,
        "col": 1,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1.0w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4144,
        "byte_start": 4099,
        "col": 1,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4144,
        "byte_start": 4099,
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
        "byte_end": 4144,
        "byte_start": 4099,
        "col": 1,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1e0w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4187,
        "byte_start": 4145,
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
        "byte_end": 4187,
        "byte_start": 4145,
        "col": 1,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1e0w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4233,
        "byte_start": 4188,
        "col": 1,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4233,
        "byte_start": 4188,
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
        "byte_end": 4233,
        "byte_start": 4188,
        "col": 1,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1www” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4276,
        "byte_start": 4234,
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
        "byte_end": 4276,
        "byte_start": 4234,
        "col": 1,
        "line": 78
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1www” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4322,
        "byte_start": 4277,
        "col": 1,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4322,
        "byte_start": 4277,
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
        "byte_end": 4322,
        "byte_start": 4277,
        "col": 1,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a +1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4364,
        "byte_start": 4323,
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
        "byte_end": 4364,
        "byte_start": 4323,
        "col": 1,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w +1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4409,
        "byte_start": 4365,
        "col": 1,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4409,
        "byte_start": 4365,
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
        "byte_end": 4409,
        "byte_start": 4365,
        "col": 1,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1W” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4450,
        "byte_start": 4410,
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
        "byte_end": 4450,
        "byte_start": 4410,
        "col": 1,
        "line": 82
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1W” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4494,
        "byte_start": 4451,
        "col": 1,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4494,
        "byte_start": 4451,
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
        "byte_end": 4494,
        "byte_start": 4451,
        "col": 1,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a Infinityw” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4542,
        "byte_start": 4495,
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
        "byte_end": 4542,
        "byte_start": 4495,
        "col": 1,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w Infinityw” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4593,
        "byte_start": 4543,
        "col": 1,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4593,
        "byte_start": 4543,
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
        "byte_end": 4593,
        "byte_start": 4543,
        "col": 1,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a NaNw” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4636,
        "byte_start": 4594,
        "col": 1,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4636,
        "byte_start": 4594,
        "col": 1,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w NaNw” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4682,
        "byte_start": 4637,
        "col": 1,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 4682,
        "byte_start": 4637,
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
        "byte_end": 4682,
        "byte_start": 4637,
        "col": 1,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0x1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4725,
        "byte_start": 4683,
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
        "byte_end": 4725,
        "byte_start": 4683,
        "col": 1,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0X1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4768,
        "byte_start": 4726,
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
        "byte_end": 4768,
        "byte_start": 4726,
        "col": 1,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1\u0001w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4842,
        "byte_start": 4769,
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
        "byte_end": 4842,
        "byte_start": 4769,
        "col": 1,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0001).",
      "severity": "Warning",
      "span": {
        "byte_end": 4779,
        "byte_start": 4778,
        "col": 1,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4917,
        "byte_start": 4843,
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
        "byte_end": 4917,
        "byte_start": 4843,
        "col": 1,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4994,
        "byte_start": 4918,
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
        "byte_end": 4994,
        "byte_start": 4918,
        "col": 1,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5071,
        "byte_start": 4995,
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
        "byte_end": 5071,
        "byte_start": 4995,
        "col": 1,
        "line": 93
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 5071,
        "byte_start": 4995,
        "col": 1,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5148,
        "byte_start": 5072,
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
        "byte_end": 5148,
        "byte_start": 5072,
        "col": 1,
        "line": 94
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 5148,
        "byte_start": 5072,
        "col": 1,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5225,
        "byte_start": 5149,
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
        "byte_end": 5225,
        "byte_start": 5149,
        "col": 1,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5302,
        "byte_start": 5226,
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
        "byte_end": 5302,
        "byte_start": 5226,
        "col": 1,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5379,
        "byte_start": 5303,
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
        "byte_end": 5379,
        "byte_start": 5303,
        "col": 1,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5456,
        "byte_start": 5380,
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
        "byte_end": 5456,
        "byte_start": 5380,
        "col": 1,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5533,
        "byte_start": 5457,
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
        "byte_end": 5533,
        "byte_start": 5457,
        "col": 1,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5610,
        "byte_start": 5534,
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
        "byte_end": 5610,
        "byte_start": 5534,
        "col": 1,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5687,
        "byte_start": 5611,
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
        "byte_end": 5687,
        "byte_start": 5611,
        "col": 1,
        "line": 101
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5764,
        "byte_start": 5688,
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
        "byte_end": 5764,
        "byte_start": 5688,
        "col": 1,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5841,
        "byte_start": 5765,
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
        "byte_end": 5841,
        "byte_start": 5765,
        "col": 1,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1‌w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5918,
        "byte_start": 5842,
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
        "byte_end": 5918,
        "byte_start": 5842,
        "col": 1,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1‍w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5995,
        "byte_start": 5919,
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
        "byte_end": 5995,
        "byte_start": 5919,
        "col": 1,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6072,
        "byte_start": 5996,
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
        "byte_end": 6072,
        "byte_start": 5996,
        "col": 1,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6149,
        "byte_start": 6073,
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
        "byte_end": 6149,
        "byte_start": 6073,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1　w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6226,
        "byte_start": 6150,
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
        "byte_end": 6226,
        "byte_start": 6150,
        "col": 1,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1﻿w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6303,
        "byte_start": 6227,
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
        "byte_end": 6303,
        "byte_start": 6227,
        "col": 1,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a \u00011w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6376,
        "byte_start": 6304,
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
        "byte_end": 6376,
        "byte_start": 6304,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0001).",
      "severity": "Warning",
      "span": {
        "byte_end": 6313,
        "byte_start": 6312,
        "col": 1,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6450,
        "byte_start": 6377,
        "col": 1,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6450,
        "byte_start": 6377,
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
        "byte_end": 6450,
        "byte_start": 6377,
        "col": 1,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6526,
        "byte_start": 6451,
        "col": 1,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6526,
        "byte_start": 6451,
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
        "byte_end": 6526,
        "byte_start": 6451,
        "col": 1,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6602,
        "byte_start": 6527,
        "col": 1,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6602,
        "byte_start": 6527,
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
        "byte_end": 6602,
        "byte_start": 6527,
        "col": 1,
        "line": 113
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 6602,
        "byte_start": 6527,
        "col": 1,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6678,
        "byte_start": 6603,
        "col": 1,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6678,
        "byte_start": 6603,
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
        "byte_end": 6678,
        "byte_start": 6603,
        "col": 1,
        "line": 114
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 6678,
        "byte_start": 6603,
        "col": 1,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6754,
        "byte_start": 6679,
        "col": 1,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6754,
        "byte_start": 6679,
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
        "byte_end": 6754,
        "byte_start": 6679,
        "col": 1,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6830,
        "byte_start": 6755,
        "col": 1,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6830,
        "byte_start": 6755,
        "col": 1,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6830,
        "byte_start": 6755,
        "col": 1,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6906,
        "byte_start": 6831,
        "col": 1,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6906,
        "byte_start": 6831,
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
        "byte_end": 6906,
        "byte_start": 6831,
        "col": 1,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6982,
        "byte_start": 6907,
        "col": 1,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 6982,
        "byte_start": 6907,
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
        "byte_end": 6982,
        "byte_start": 6907,
        "col": 1,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7058,
        "byte_start": 6983,
        "col": 1,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7058,
        "byte_start": 6983,
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
        "byte_end": 7058,
        "byte_start": 6983,
        "col": 1,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7134,
        "byte_start": 7059,
        "col": 1,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7134,
        "byte_start": 7059,
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
        "byte_end": 7134,
        "byte_start": 7059,
        "col": 1,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7210,
        "byte_start": 7135,
        "col": 1,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7210,
        "byte_start": 7135,
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
        "byte_end": 7210,
        "byte_start": 7135,
        "col": 1,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7286,
        "byte_start": 7211,
        "col": 1,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7286,
        "byte_start": 7211,
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
        "byte_end": 7286,
        "byte_start": 7211,
        "col": 1,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7362,
        "byte_start": 7287,
        "col": 1,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7362,
        "byte_start": 7287,
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
        "byte_end": 7362,
        "byte_start": 7287,
        "col": 1,
        "line": 123
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ‌1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7438,
        "byte_start": 7363,
        "col": 1,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7438,
        "byte_start": 7363,
        "col": 1,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ‍1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7514,
        "byte_start": 7439,
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
        "byte_end": 7514,
        "byte_start": 7439,
        "col": 1,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7590,
        "byte_start": 7515,
        "col": 1,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7590,
        "byte_start": 7515,
        "col": 1,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7590,
        "byte_start": 7515,
        "col": 1,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7666,
        "byte_start": 7591,
        "col": 1,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7666,
        "byte_start": 7591,
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
        "byte_end": 7666,
        "byte_start": 7591,
        "col": 1,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 　1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7742,
        "byte_start": 7667,
        "col": 1,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 7742,
        "byte_start": 7667,
        "col": 1,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7742,
        "byte_start": 7667,
        "col": 1,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ﻿1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7818,
        "byte_start": 7743,
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
        "byte_end": 7818,
        "byte_start": 7743,
        "col": 1,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7866,
        "byte_start": 7819,
        "col": 1,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7866,
        "byte_start": 7819,
        "col": 1,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a -0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7915,
        "byte_start": 7867,
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
        "byte_end": 7915,
        "byte_start": 7867,
        "col": 1,
        "line": 131
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x -0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7960,
        "byte_start": 7916,
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
        "byte_end": 7960,
        "byte_start": 7916,
        "col": 1,
        "line": 132
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a -1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8002,
        "byte_start": 7961,
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
        "byte_end": 8002,
        "byte_start": 7961,
        "col": 1,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x -1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8047,
        "byte_start": 8003,
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
        "byte_end": 8047,
        "byte_start": 8003,
        "col": 1,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1e0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8097,
        "byte_start": 8048,
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
        "byte_end": 8097,
        "byte_start": 8048,
        "col": 1,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1E0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8147,
        "byte_start": 8098,
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
        "byte_end": 8147,
        "byte_start": 8098,
        "col": 1,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1e-1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8198,
        "byte_start": 8148,
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
        "byte_end": 8198,
        "byte_start": 8148,
        "col": 1,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1.5e1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8250,
        "byte_start": 8199,
        "col": 1,
        "line": 138
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8250,
        "byte_start": 8199,
        "col": 1,
        "line": 138
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a -x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8291,
        "byte_start": 8251,
        "col": 1,
        "line": 139
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8291,
        "byte_start": 8251,
        "col": 1,
        "line": 139
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a .x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8332,
        "byte_start": 8292,
        "col": 1,
        "line": 140
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8332,
        "byte_start": 8292,
        "col": 1,
        "line": 140
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a -.x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8374,
        "byte_start": 8333,
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
        "byte_end": 8374,
        "byte_start": 8333,
        "col": 1,
        "line": 141
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1.x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8416,
        "byte_start": 8375,
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
        "byte_end": 8416,
        "byte_start": 8375,
        "col": 1,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a .5x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8465,
        "byte_start": 8417,
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
        "byte_end": 8465,
        "byte_start": 8417,
        "col": 1,
        "line": 143
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a .5e1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8516,
        "byte_start": 8466,
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
        "byte_end": 8516,
        "byte_start": 8466,
        "col": 1,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x 1.5e1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8564,
        "byte_start": 8517,
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
        "byte_end": 8564,
        "byte_start": 8517,
        "col": 1,
        "line": 145
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x 1e1.5x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8612,
        "byte_start": 8565,
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
        "byte_end": 8612,
        "byte_start": 8565,
        "col": 1,
        "line": 146
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1.0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8662,
        "byte_start": 8613,
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
        "byte_end": 8662,
        "byte_start": 8613,
        "col": 1,
        "line": 147
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1x 1.0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8708,
        "byte_start": 8663,
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
        "byte_end": 8708,
        "byte_start": 8663,
        "col": 1,
        "line": 148
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a +1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8750,
        "byte_start": 8709,
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
        "byte_end": 8750,
        "byte_start": 8709,
        "col": 1,
        "line": 149
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1X” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8791,
        "byte_start": 8751,
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
        "byte_end": 8791,
        "byte_start": 8751,
        "col": 1,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a Infinityx” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8839,
        "byte_start": 8792,
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
        "byte_end": 8839,
        "byte_start": 8792,
        "col": 1,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a NaNx” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8882,
        "byte_start": 8840,
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
        "byte_end": 8882,
        "byte_start": 8840,
        "col": 1,
        "line": 152
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0x1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8925,
        "byte_start": 8883,
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
        "byte_end": 8925,
        "byte_start": 8883,
        "col": 1,
        "line": 153
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0X1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8968,
        "byte_start": 8926,
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
        "byte_end": 8968,
        "byte_start": 8926,
        "col": 1,
        "line": 154
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1\u0001x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9042,
        "byte_start": 8969,
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
        "byte_end": 9042,
        "byte_start": 8969,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0001).",
      "severity": "Warning",
      "span": {
        "byte_end": 8979,
        "byte_start": 8978,
        "col": 1,
        "line": 155
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9117,
        "byte_start": 9043,
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
        "byte_end": 9117,
        "byte_start": 9043,
        "col": 1,
        "line": 156
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9194,
        "byte_start": 9118,
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
        "byte_end": 9194,
        "byte_start": 9118,
        "col": 1,
        "line": 157
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9271,
        "byte_start": 9195,
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
        "byte_end": 9271,
        "byte_start": 9195,
        "col": 1,
        "line": 158
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 9271,
        "byte_start": 9195,
        "col": 1,
        "line": 158
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9348,
        "byte_start": 9272,
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
        "byte_end": 9348,
        "byte_start": 9272,
        "col": 1,
        "line": 159
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 9348,
        "byte_start": 9272,
        "col": 1,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9425,
        "byte_start": 9349,
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
        "byte_end": 9425,
        "byte_start": 9349,
        "col": 1,
        "line": 160
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9502,
        "byte_start": 9426,
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
        "byte_end": 9502,
        "byte_start": 9426,
        "col": 1,
        "line": 161
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9579,
        "byte_start": 9503,
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
        "byte_end": 9579,
        "byte_start": 9503,
        "col": 1,
        "line": 162
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9656,
        "byte_start": 9580,
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
        "byte_end": 9656,
        "byte_start": 9580,
        "col": 1,
        "line": 163
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9733,
        "byte_start": 9657,
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
        "byte_end": 9733,
        "byte_start": 9657,
        "col": 1,
        "line": 164
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9810,
        "byte_start": 9734,
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
        "byte_end": 9810,
        "byte_start": 9734,
        "col": 1,
        "line": 165
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9887,
        "byte_start": 9811,
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
        "byte_end": 9887,
        "byte_start": 9811,
        "col": 1,
        "line": 166
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9964,
        "byte_start": 9888,
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
        "byte_end": 9964,
        "byte_start": 9888,
        "col": 1,
        "line": 167
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10041,
        "byte_start": 9965,
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
        "byte_end": 10041,
        "byte_start": 9965,
        "col": 1,
        "line": 168
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1‌x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10118,
        "byte_start": 10042,
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
        "byte_end": 10118,
        "byte_start": 10042,
        "col": 1,
        "line": 169
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1‍x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10195,
        "byte_start": 10119,
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
        "byte_end": 10195,
        "byte_start": 10119,
        "col": 1,
        "line": 170
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10272,
        "byte_start": 10196,
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
        "byte_end": 10272,
        "byte_start": 10196,
        "col": 1,
        "line": 171
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1 x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10349,
        "byte_start": 10273,
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
        "byte_end": 10349,
        "byte_start": 10273,
        "col": 1,
        "line": 172
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1　x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10426,
        "byte_start": 10350,
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
        "byte_end": 10426,
        "byte_start": 10350,
        "col": 1,
        "line": 173
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1﻿x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10503,
        "byte_start": 10427,
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
        "byte_end": 10503,
        "byte_start": 10427,
        "col": 1,
        "line": 174
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a \u00011x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10576,
        "byte_start": 10504,
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
        "byte_end": 10576,
        "byte_start": 10504,
        "col": 1,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0001).",
      "severity": "Warning",
      "span": {
        "byte_end": 10513,
        "byte_start": 10512,
        "col": 1,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10650,
        "byte_start": 10577,
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
        "byte_end": 10650,
        "byte_start": 10577,
        "col": 1,
        "line": 176
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10726,
        "byte_start": 10651,
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
        "byte_end": 10726,
        "byte_start": 10651,
        "col": 1,
        "line": 177
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10802,
        "byte_start": 10727,
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
        "byte_end": 10802,
        "byte_start": 10727,
        "col": 1,
        "line": 178
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 10802,
        "byte_start": 10727,
        "col": 1,
        "line": 178
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10878,
        "byte_start": 10803,
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
        "byte_end": 10878,
        "byte_start": 10803,
        "col": 1,
        "line": 179
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 10878,
        "byte_start": 10803,
        "col": 1,
        "line": 179
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10954,
        "byte_start": 10879,
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
        "byte_end": 10954,
        "byte_start": 10879,
        "col": 1,
        "line": 180
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11030,
        "byte_start": 10955,
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
        "byte_end": 11030,
        "byte_start": 10955,
        "col": 1,
        "line": 181
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11106,
        "byte_start": 11031,
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
        "byte_end": 11106,
        "byte_start": 11031,
        "col": 1,
        "line": 182
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11182,
        "byte_start": 11107,
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
        "byte_end": 11182,
        "byte_start": 11107,
        "col": 1,
        "line": 183
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11258,
        "byte_start": 11183,
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
        "byte_end": 11258,
        "byte_start": 11183,
        "col": 1,
        "line": 184
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11334,
        "byte_start": 11259,
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
        "byte_end": 11334,
        "byte_start": 11259,
        "col": 1,
        "line": 185
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11410,
        "byte_start": 11335,
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
        "byte_end": 11410,
        "byte_start": 11335,
        "col": 1,
        "line": 186
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11486,
        "byte_start": 11411,
        "col": 1,
        "line": 187
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11486,
        "byte_start": 11411,
        "col": 1,
        "line": 187
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11562,
        "byte_start": 11487,
        "col": 1,
        "line": 188
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11562,
        "byte_start": 11487,
        "col": 1,
        "line": 188
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ‌1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11638,
        "byte_start": 11563,
        "col": 1,
        "line": 189
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11638,
        "byte_start": 11563,
        "col": 1,
        "line": 189
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ‍1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11714,
        "byte_start": 11639,
        "col": 1,
        "line": 190
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11714,
        "byte_start": 11639,
        "col": 1,
        "line": 190
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11790,
        "byte_start": 11715,
        "col": 1,
        "line": 191
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11790,
        "byte_start": 11715,
        "col": 1,
        "line": 191
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a  1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11866,
        "byte_start": 11791,
        "col": 1,
        "line": 192
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11866,
        "byte_start": 11791,
        "col": 1,
        "line": 192
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 　1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 11942,
        "byte_start": 11867,
        "col": 1,
        "line": 193
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 11942,
        "byte_start": 11867,
        "col": 1,
        "line": 193
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a ﻿1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12018,
        "byte_start": 11943,
        "col": 1,
        "line": 194
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12018,
        "byte_start": 11943,
        "col": 1,
        "line": 194
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 0h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12062,
        "byte_start": 12019,
        "col": 1,
        "line": 195
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12062,
        "byte_start": 12019,
        "col": 1,
        "line": 195
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12062,
        "byte_start": 12019,
        "col": 1,
        "line": 195
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w -1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12107,
        "byte_start": 12063,
        "col": 1,
        "line": 196
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12107,
        "byte_start": 12063,
        "col": 1,
        "line": 196
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12107,
        "byte_start": 12063,
        "col": 1,
        "line": 196
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1.0h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12153,
        "byte_start": 12108,
        "col": 1,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12153,
        "byte_start": 12108,
        "col": 1,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12153,
        "byte_start": 12108,
        "col": 1,
        "line": 197
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1e0h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12199,
        "byte_start": 12154,
        "col": 1,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12199,
        "byte_start": 12154,
        "col": 1,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12199,
        "byte_start": 12154,
        "col": 1,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1hhh” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12245,
        "byte_start": 12200,
        "col": 1,
        "line": 199
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12245,
        "byte_start": 12200,
        "col": 1,
        "line": 199
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12245,
        "byte_start": 12200,
        "col": 1,
        "line": 199
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w +1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12290,
        "byte_start": 12246,
        "col": 1,
        "line": 200
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12290,
        "byte_start": 12246,
        "col": 1,
        "line": 200
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12290,
        "byte_start": 12246,
        "col": 1,
        "line": 200
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1H” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12334,
        "byte_start": 12291,
        "col": 1,
        "line": 201
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12334,
        "byte_start": 12291,
        "col": 1,
        "line": 201
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12334,
        "byte_start": 12291,
        "col": 1,
        "line": 201
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w Infinityh” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12385,
        "byte_start": 12335,
        "col": 1,
        "line": 202
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12385,
        "byte_start": 12335,
        "col": 1,
        "line": 202
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12385,
        "byte_start": 12335,
        "col": 1,
        "line": 202
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w NaNh” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12431,
        "byte_start": 12386,
        "col": 1,
        "line": 203
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12431,
        "byte_start": 12386,
        "col": 1,
        "line": 203
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12431,
        "byte_start": 12386,
        "col": 1,
        "line": 203
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0x1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12474,
        "byte_start": 12432,
        "col": 1,
        "line": 204
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12474,
        "byte_start": 12432,
        "col": 1,
        "line": 204
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 0X1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12517,
        "byte_start": 12475,
        "col": 1,
        "line": 205
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12517,
        "byte_start": 12475,
        "col": 1,
        "line": 205
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1\u0001h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12594,
        "byte_start": 12518,
        "col": 1,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12594,
        "byte_start": 12518,
        "col": 1,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12594,
        "byte_start": 12518,
        "col": 1,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0001).",
      "severity": "Warning",
      "span": {
        "byte_end": 12531,
        "byte_start": 12530,
        "col": 1,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12672,
        "byte_start": 12595,
        "col": 1,
        "line": 207
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12672,
        "byte_start": 12595,
        "col": 1,
        "line": 207
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12672,
        "byte_start": 12595,
        "col": 1,
        "line": 207
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12752,
        "byte_start": 12673,
        "col": 1,
        "line": 208
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12752,
        "byte_start": 12673,
        "col": 1,
        "line": 208
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12752,
        "byte_start": 12673,
        "col": 1,
        "line": 208
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12832,
        "byte_start": 12753,
        "col": 1,
        "line": 209
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12832,
        "byte_start": 12753,
        "col": 1,
        "line": 209
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12832,
        "byte_start": 12753,
        "col": 1,
        "line": 209
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 12832,
        "byte_start": 12753,
        "col": 1,
        "line": 209
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12912,
        "byte_start": 12833,
        "col": 1,
        "line": 210
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12912,
        "byte_start": 12833,
        "col": 1,
        "line": 210
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12912,
        "byte_start": 12833,
        "col": 1,
        "line": 210
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 12912,
        "byte_start": 12833,
        "col": 1,
        "line": 210
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 12992,
        "byte_start": 12913,
        "col": 1,
        "line": 211
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 12992,
        "byte_start": 12913,
        "col": 1,
        "line": 211
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 12992,
        "byte_start": 12913,
        "col": 1,
        "line": 211
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13072,
        "byte_start": 12993,
        "col": 1,
        "line": 212
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13072,
        "byte_start": 12993,
        "col": 1,
        "line": 212
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13072,
        "byte_start": 12993,
        "col": 1,
        "line": 212
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13152,
        "byte_start": 13073,
        "col": 1,
        "line": 213
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13152,
        "byte_start": 13073,
        "col": 1,
        "line": 213
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13152,
        "byte_start": 13073,
        "col": 1,
        "line": 213
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13232,
        "byte_start": 13153,
        "col": 1,
        "line": 214
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13232,
        "byte_start": 13153,
        "col": 1,
        "line": 214
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13232,
        "byte_start": 13153,
        "col": 1,
        "line": 214
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13312,
        "byte_start": 13233,
        "col": 1,
        "line": 215
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13312,
        "byte_start": 13233,
        "col": 1,
        "line": 215
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13312,
        "byte_start": 13233,
        "col": 1,
        "line": 215
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13392,
        "byte_start": 13313,
        "col": 1,
        "line": 216
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13392,
        "byte_start": 13313,
        "col": 1,
        "line": 216
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13392,
        "byte_start": 13313,
        "col": 1,
        "line": 216
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13472,
        "byte_start": 13393,
        "col": 1,
        "line": 217
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13472,
        "byte_start": 13393,
        "col": 1,
        "line": 217
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13472,
        "byte_start": 13393,
        "col": 1,
        "line": 217
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13552,
        "byte_start": 13473,
        "col": 1,
        "line": 218
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13552,
        "byte_start": 13473,
        "col": 1,
        "line": 218
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13552,
        "byte_start": 13473,
        "col": 1,
        "line": 218
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13632,
        "byte_start": 13553,
        "col": 1,
        "line": 219
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13632,
        "byte_start": 13553,
        "col": 1,
        "line": 219
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13632,
        "byte_start": 13553,
        "col": 1,
        "line": 219
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1‌h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13712,
        "byte_start": 13633,
        "col": 1,
        "line": 220
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13712,
        "byte_start": 13633,
        "col": 1,
        "line": 220
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13712,
        "byte_start": 13633,
        "col": 1,
        "line": 220
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1‍h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13792,
        "byte_start": 13713,
        "col": 1,
        "line": 221
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13792,
        "byte_start": 13713,
        "col": 1,
        "line": 221
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13792,
        "byte_start": 13713,
        "col": 1,
        "line": 221
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13872,
        "byte_start": 13793,
        "col": 1,
        "line": 222
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13872,
        "byte_start": 13793,
        "col": 1,
        "line": 222
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13872,
        "byte_start": 13793,
        "col": 1,
        "line": 222
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1 h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 13952,
        "byte_start": 13873,
        "col": 1,
        "line": 223
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 13952,
        "byte_start": 13873,
        "col": 1,
        "line": 223
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 13952,
        "byte_start": 13873,
        "col": 1,
        "line": 223
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1　h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14032,
        "byte_start": 13953,
        "col": 1,
        "line": 224
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14032,
        "byte_start": 13953,
        "col": 1,
        "line": 224
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14032,
        "byte_start": 13953,
        "col": 1,
        "line": 224
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 1﻿h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14112,
        "byte_start": 14033,
        "col": 1,
        "line": 225
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14112,
        "byte_start": 14033,
        "col": 1,
        "line": 225
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14112,
        "byte_start": 14033,
        "col": 1,
        "line": 225
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w \u00011h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14188,
        "byte_start": 14113,
        "col": 1,
        "line": 226
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14188,
        "byte_start": 14113,
        "col": 1,
        "line": 226
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14188,
        "byte_start": 14113,
        "col": 1,
        "line": 226
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_control",
      "message": "Character reference expands to a control character (U+0001).",
      "severity": "Warning",
      "span": {
        "byte_end": 14125,
        "byte_start": 14124,
        "col": 1,
        "line": 226
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14265,
        "byte_start": 14189,
        "col": 1,
        "line": 227
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14265,
        "byte_start": 14189,
        "col": 1,
        "line": 227
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14265,
        "byte_start": 14189,
        "col": 1,
        "line": 227
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14344,
        "byte_start": 14266,
        "col": 1,
        "line": 228
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14344,
        "byte_start": 14266,
        "col": 1,
        "line": 228
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14344,
        "byte_start": 14266,
        "col": 1,
        "line": 228
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14423,
        "byte_start": 14345,
        "col": 1,
        "line": 229
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14423,
        "byte_start": 14345,
        "col": 1,
        "line": 229
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14423,
        "byte_start": 14345,
        "col": 1,
        "line": 229
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 14423,
        "byte_start": 14345,
        "col": 1,
        "line": 229
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14502,
        "byte_start": 14424,
        "col": 1,
        "line": 230
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14502,
        "byte_start": 14424,
        "col": 1,
        "line": 230
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14502,
        "byte_start": 14424,
        "col": 1,
        "line": 230
      }
    },
    {
      "category": "I18n",
      "code": "i18n.unicode.nfc.attribute_value_not_nfc",
      "message": "The value of attribute “srcset” on element “img” is not in Unicode Normalization Form C.",
      "severity": "Warning",
      "span": {
        "byte_end": 14502,
        "byte_start": 14424,
        "col": 1,
        "line": 230
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14581,
        "byte_start": 14503,
        "col": 1,
        "line": 231
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14581,
        "byte_start": 14503,
        "col": 1,
        "line": 231
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14581,
        "byte_start": 14503,
        "col": 1,
        "line": 231
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14660,
        "byte_start": 14582,
        "col": 1,
        "line": 232
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14660,
        "byte_start": 14582,
        "col": 1,
        "line": 232
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14660,
        "byte_start": 14582,
        "col": 1,
        "line": 232
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14739,
        "byte_start": 14661,
        "col": 1,
        "line": 233
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14739,
        "byte_start": 14661,
        "col": 1,
        "line": 233
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14739,
        "byte_start": 14661,
        "col": 1,
        "line": 233
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14818,
        "byte_start": 14740,
        "col": 1,
        "line": 234
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14818,
        "byte_start": 14740,
        "col": 1,
        "line": 234
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14818,
        "byte_start": 14740,
        "col": 1,
        "line": 234
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14897,
        "byte_start": 14819,
        "col": 1,
        "line": 235
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14897,
        "byte_start": 14819,
        "col": 1,
        "line": 235
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14897,
        "byte_start": 14819,
        "col": 1,
        "line": 235
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 14976,
        "byte_start": 14898,
        "col": 1,
        "line": 236
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 14976,
        "byte_start": 14898,
        "col": 1,
        "line": 236
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 14976,
        "byte_start": 14898,
        "col": 1,
        "line": 236
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15055,
        "byte_start": 14977,
        "col": 1,
        "line": 237
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15055,
        "byte_start": 14977,
        "col": 1,
        "line": 237
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15055,
        "byte_start": 14977,
        "col": 1,
        "line": 237
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15134,
        "byte_start": 15056,
        "col": 1,
        "line": 238
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15134,
        "byte_start": 15056,
        "col": 1,
        "line": 238
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15134,
        "byte_start": 15056,
        "col": 1,
        "line": 238
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15213,
        "byte_start": 15135,
        "col": 1,
        "line": 239
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15213,
        "byte_start": 15135,
        "col": 1,
        "line": 239
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15213,
        "byte_start": 15135,
        "col": 1,
        "line": 239
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w ‌1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15292,
        "byte_start": 15214,
        "col": 1,
        "line": 240
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15292,
        "byte_start": 15214,
        "col": 1,
        "line": 240
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15292,
        "byte_start": 15214,
        "col": 1,
        "line": 240
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w ‍1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15371,
        "byte_start": 15293,
        "col": 1,
        "line": 241
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15371,
        "byte_start": 15293,
        "col": 1,
        "line": 241
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15371,
        "byte_start": 15293,
        "col": 1,
        "line": 241
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15450,
        "byte_start": 15372,
        "col": 1,
        "line": 242
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15450,
        "byte_start": 15372,
        "col": 1,
        "line": 242
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15450,
        "byte_start": 15372,
        "col": 1,
        "line": 242
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w  1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15529,
        "byte_start": 15451,
        "col": 1,
        "line": 243
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15529,
        "byte_start": 15451,
        "col": 1,
        "line": 243
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15529,
        "byte_start": 15451,
        "col": 1,
        "line": 243
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w 　1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15608,
        "byte_start": 15530,
        "col": 1,
        "line": 244
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15608,
        "byte_start": 15530,
        "col": 1,
        "line": 244
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15608,
        "byte_start": 15530,
        "col": 1,
        "line": 244
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a 1w ﻿1h” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 15687,
        "byte_start": 15609,
        "col": 1,
        "line": 245
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.width_descriptor_requires_sizes",
      "message": "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 15687,
        "byte_start": 15609,
        "col": 1,
        "line": 245
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 15687,
        "byte_start": 15609,
        "col": 1,
        "line": 245
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/srcset/parse-a-srcset-attribute.html"
}
```
