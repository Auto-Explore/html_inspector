# html/semantics/embedded-content/the-img-element/update-the-source-set.html

Counts:
- errors: 1
- warnings: 210
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-source-set.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>img update the source set</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
setup({explicit_done:true});

function check(p) {
  var img = p.querySelector('[data-expect]');
  test(function() {
    var expect = img.dataset.expect;
    if ('resolve' in img.dataset) {
      var a = document.createElement('a');
      a.href = expect;
      expect = a.href;
    }
    assert_equals(img.currentSrc, expect);
  }, p.innerHTML);
}

onload = function() {
  [].forEach.call(document.querySelectorAll('div:not([id])'), check);
  done();
};

</script>
<div id=log></div>
<div><img data-expect=''></div>
<div><img src data-expect=''></div>
<div><img src='data:,a' data-expect='data:,a'></div>
<div><img srcset src='data:,a' data-expect='data:,a'></div>
<div><img srcset='data:,b' src='data:,a' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b' data-expect='data:,b'><!-- srcset after src --></div>
<div><img src='data:,a' srcset='data:,b 1x' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 1.0x' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 1e0x' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 10000w' sizes='1px' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 10000w, data:,c 10000x' sizes='1px' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 10000x, data:,c 10000w' sizes='1px' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 1w' sizes='10000px' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 1w, data:,c 0.0001x' sizes='10000px' data-expect='data:,b'></div>
<div><img src='data:,a' srcset='data:,b 0.0001x, data:,c 1w' sizes='10000px' data-expect='data:,b'></div>
<div><img srcset='data:,a' data-expect='data:,a'></div>

<!-- child is not a <source> -->

<div><picture>foo<img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><!--foo--><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><br><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><p></p><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><video><source srcset='data:,b'></video><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><span><source srcset='data:,b'></span><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><svg><source srcset='data:,b'/></svg><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><svg/><source srcset='data:,b'/><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><svg><font/><source srcset='data:,b'/></svg><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><svg><!--<font face> tag breaks out of svg--><font face></font><source srcset='data:,b'/></svg><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><img src='data:,a'><img src='data:,b' data-expect='data:,b'></picture></div>

<!-- <source> has no srcset -->

<div><picture><source><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source src='data:,b'><img src='data:,a' data-expect='data:,a'></picture></div>

<!-- <source srcset> has zero candidates -->

<div><picture><source srcset><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset=', ,'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b 1x 1x'><img src='data:,a' data-expect='data:,a'></picture></div>

<!-- <source media> -->

<div><picture><source srcset='data:,b' media><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' media='all'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' media='all and (min-width:0)'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' media='all and !'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media='all and (!)'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media='not all'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media='not all and (min-width:0)'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media='not all and (max-width:0)'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' media='not all and !'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media='not all and (!)'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media='all, !'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' media=','><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' media=', all'><img src='data:,a' data-expect='data:,b'></picture></div>

<!-- <source type> assume support for gif, png, jpg, svg, ico -->

<div><picture><source srcset='data:,b' type><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type=' '><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type=' image/gif'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif '><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif;'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif;encodings'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif;encodings='><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif;encodings=foobar'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/png'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/jpeg'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/svg+xml'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='image/x-icon'><img src='data:,a' data-expect='data:,b'></picture></div>
<div><picture><source srcset='data:,b' type='text/xml'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='text/html'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='text/plain'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='text/css'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='video/mp4'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='video/ogg'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='video/webm'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='unknown/unknown'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='application/octet-stream'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='application/x-shockwave-flash'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='image\gif'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='gif'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='.gif'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='*'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='*/*'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='image/*'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type=','><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif, image/png'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='image/gif image/png'><img src='data:,a' data-expect='data:,a'></picture></div>
<div><picture><source srcset='data:,b' type='image/foobarbaz'><img src='data:,a' data-expect='data:,a'></picture></div>

<!-- trailing garbage -->

<div><picture><img src='data:,a' data-expect='data:,a'>foo</picture></div>
<div><picture><img src='data:,a' data-expect='data:,a'><br></picture></div>
<div><picture><img src='data:,a' data-expect='data:,a'><!--foo--></picture></div>
<div><picture><img src='data:,a' data-expect='data:,a'><img src='data:,b'></picture></div>
<div><picture><img data-expect=''><img src='data:,b'></picture></div>
<div><picture><img src='data:,a' data-expect='data:,a'><source srcset='data:,b'></picture></div>
<div><picture><img data-expect=''><source srcset='data:,b'></picture></div>

<!-- parent not picture -->

<div><picture><span><source srcset='data:,b'><img data-expect=''></span></picture></div>
<div><picture><span><source srcset='data:,b'><img src='data:,a' data-expect='data:,a'></span></picture></div>
<div><picture><source srcset='data:,b'><span><img src='data:,a' data-expect='data:,a'></span></picture></div>

<!-- no src -->

<div><picture><source srcset='data:,b'><img data-expect='data:,b'></picture></div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 681,
        "byte_start": 661,
        "col": 6,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 681,
        "byte_start": 661,
        "col": 6,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 717,
        "byte_start": 693,
        "col": 6,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 717,
        "byte_start": 693,
        "col": 6,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 770,
        "byte_start": 729,
        "col": 6,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 830,
        "byte_start": 782,
        "col": 6,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 830,
        "byte_start": 782,
        "col": 6,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 900,
        "byte_start": 842,
        "col": 6,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 900,
        "byte_start": 842,
        "col": 6,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 970,
        "byte_start": 912,
        "col": 6,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 970,
        "byte_start": 912,
        "col": 6,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 1x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1068,
        "byte_start": 1007,
        "col": 6,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1068,
        "byte_start": 1007,
        "col": 6,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 1.0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1143,
        "byte_start": 1080,
        "col": 6,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1143,
        "byte_start": 1080,
        "col": 6,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 1e0x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1218,
        "byte_start": 1155,
        "col": 6,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1218,
        "byte_start": 1155,
        "col": 6,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 10000w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1307,
        "byte_start": 1230,
        "col": 6,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1307,
        "byte_start": 1230,
        "col": 6,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 10000w, data:,c 10000x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1412,
        "byte_start": 1319,
        "col": 6,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1412,
        "byte_start": 1319,
        "col": 6,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 10000x, data:,c 10000w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1517,
        "byte_start": 1424,
        "col": 6,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1517,
        "byte_start": 1424,
        "col": 6,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1606,
        "byte_start": 1529,
        "col": 6,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1606,
        "byte_start": 1529,
        "col": 6,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 1w, data:,c 0.0001x” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1712,
        "byte_start": 1618,
        "col": 6,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1712,
        "byte_start": 1618,
        "col": 6,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,b 0.0001x, data:,c 1w” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1818,
        "byte_start": 1724,
        "col": 6,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1818,
        "byte_start": 1724,
        "col": 6,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.img.srcset.invalid",
      "message": "Bad value “data:,a” for attribute “srcset” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1874,
        "byte_start": 1830,
        "col": 6,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1874,
        "byte_start": 1830,
        "col": 6,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.picture.text.disallowed",
      "message": "Text not allowed in “picture” in this context.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1974,
        "byte_start": 1933,
        "col": 18,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2056,
        "byte_start": 2015,
        "col": 25,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “br” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2091,
        "byte_start": 2087,
        "col": 15,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2132,
        "byte_start": 2091,
        "col": 19,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “p” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2166,
        "byte_start": 2163,
        "col": 15,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2211,
        "byte_start": 2170,
        "col": 22,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “video” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2249,
        "byte_start": 2242,
        "col": 15,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.video.transparent.disallowed_child_in_phrasing",
      "message": "Element “source” not allowed as child of “video” in this context. Note: The “video” element has a transparent content model; its allowed content is inherited from its parent element.",
      "severity": "Warning",
      "span": {
        "byte_end": 2274,
        "byte_start": 2249,
        "col": 22,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.source.attr.srcset.disallowed",
      "message": "Attribute “srcset” not allowed on element “source” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 2274,
        "byte_start": 2249,
        "col": 22,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2323,
        "byte_start": 2282,
        "col": 55,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2360,
        "byte_start": 2354,
        "col": 15,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2433,
        "byte_start": 2392,
        "col": 53,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “svg” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2469,
        "byte_start": 2464,
        "col": 15,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “source” not allowed as child of “svg” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2495,
        "byte_start": 2469,
        "col": 20,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “source” element is a completely-unknown element that is not allowed anywhere in any SVG content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2495,
        "byte_start": 2469,
        "col": 20,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2542,
        "byte_start": 2501,
        "col": 52,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.parse.self_closing.non_void",
      "message": "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
      "severity": "Error",
      "span": {
        "byte_end": 2579,
        "byte_start": 2573,
        "col": 15,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “svg” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2579,
        "byte_start": 2573,
        "col": 15,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2605,
        "byte_start": 2579,
        "col": 21,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2646,
        "byte_start": 2605,
        "col": 47,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “svg” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2682,
        "byte_start": 2677,
        "col": 15,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.font.missing_horiz_adv_x",
      "message": "Element “font” is missing required attribute “horiz-adv-x”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2689,
        "byte_start": 2682,
        "col": 20,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.font.missing_missing_glyph",
      "message": "Element “font” is missing a required instance of child element “missing-glyph”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2689,
        "byte_start": 2682,
        "col": 20,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “source” not allowed as child of “svg” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2715,
        "byte_start": 2689,
        "col": 27,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “source” element is a completely-unknown element that is not allowed anywhere in any SVG content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2715,
        "byte_start": 2689,
        "col": 27,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2762,
        "byte_start": 2721,
        "col": 59,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “svg” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2798,
        "byte_start": 2793,
        "col": 15,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “font” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2849,
        "byte_start": 2838,
        "col": 60,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.element.font.obsolete",
      "message": "The “font” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2849,
        "byte_start": 2838,
        "col": 60,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2882,
        "byte_start": 2856,
        "col": 78,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2929,
        "byte_start": 2888,
        "col": 110,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2979,
        "byte_start": 2960,
        "col": 15,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3020,
        "byte_start": 2979,
        "col": 34,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3020,
        "byte_start": 2979,
        "col": 34,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3093,
        "byte_start": 3085,
        "col": 15,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3134,
        "byte_start": 3093,
        "col": 23,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.source.attr.src.disallowed",
      "message": "Attribute “src” not allowed on element “source” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3187,
        "byte_start": 3165,
        "col": 15,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.source.missing_srcset",
      "message": "Element “source” is missing required attribute “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3187,
        "byte_start": 3165,
        "col": 15,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3228,
        "byte_start": 3187,
        "col": 37,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3321,
        "byte_start": 3306,
        "col": 15,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3362,
        "byte_start": 3321,
        "col": 30,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “, ,” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3414,
        "byte_start": 3393,
        "col": 15,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3455,
        "byte_start": 3414,
        "col": 36,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b 1x 1x” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3517,
        "byte_start": 3486,
        "col": 15,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3558,
        "byte_start": 3517,
        "col": 46,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3646,
        "byte_start": 3615,
        "col": 15,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.picture.source.media.empty",
      "message": "Value of “media” attribute here must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 3646,
        "byte_start": 3615,
        "col": 15,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3687,
        "byte_start": 3646,
        "col": 46,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3755,
        "byte_start": 3718,
        "col": 15,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3796,
        "byte_start": 3755,
        "col": 52,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3882,
        "byte_start": 3827,
        "col": 15,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3923,
        "byte_start": 3882,
        "col": 70,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3997,
        "byte_start": 3954,
        "col": 15,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “all and !” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3997,
        "byte_start": 3954,
        "col": 15,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4038,
        "byte_start": 3997,
        "col": 58,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4114,
        "byte_start": 4069,
        "col": 15,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “all and (!)” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4114,
        "byte_start": 4069,
        "col": 15,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4155,
        "byte_start": 4114,
        "col": 60,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4227,
        "byte_start": 4186,
        "col": 15,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4268,
        "byte_start": 4227,
        "col": 56,
        "line": 77
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4358,
        "byte_start": 4299,
        "col": 15,
        "line": 78
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4399,
        "byte_start": 4358,
        "col": 74,
        "line": 78
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4489,
        "byte_start": 4430,
        "col": 15,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4530,
        "byte_start": 4489,
        "col": 74,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4608,
        "byte_start": 4561,
        "col": 15,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “not all and !” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4608,
        "byte_start": 4561,
        "col": 15,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4649,
        "byte_start": 4608,
        "col": 62,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4729,
        "byte_start": 4680,
        "col": 15,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “not all and (!)” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4729,
        "byte_start": 4680,
        "col": 15,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4770,
        "byte_start": 4729,
        "col": 64,
        "line": 81
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4841,
        "byte_start": 4801,
        "col": 15,
        "line": 82
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “all, !” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4841,
        "byte_start": 4801,
        "col": 15,
        "line": 82
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4882,
        "byte_start": 4841,
        "col": 55,
        "line": 82
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4948,
        "byte_start": 4913,
        "col": 15,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “,” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4948,
        "byte_start": 4913,
        "col": 15,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4989,
        "byte_start": 4948,
        "col": 50,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5059,
        "byte_start": 5020,
        "col": 15,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.source.media.invalid",
      "message": "Bad value “, all” for attribute “media” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5059,
        "byte_start": 5020,
        "col": 15,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5100,
        "byte_start": 5059,
        "col": 54,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5229,
        "byte_start": 5199,
        "col": 15,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5270,
        "byte_start": 5229,
        "col": 45,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5335,
        "byte_start": 5301,
        "col": 15,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5376,
        "byte_start": 5335,
        "col": 49,
        "line": 89
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5449,
        "byte_start": 5407,
        "col": 15,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5490,
        "byte_start": 5449,
        "col": 57,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5564,
        "byte_start": 5521,
        "col": 15,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5605,
        "byte_start": 5564,
        "col": 58,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5679,
        "byte_start": 5636,
        "col": 15,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5720,
        "byte_start": 5679,
        "col": 58,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5794,
        "byte_start": 5751,
        "col": 15,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5835,
        "byte_start": 5794,
        "col": 58,
        "line": 93
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5918,
        "byte_start": 5866,
        "col": 15,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5959,
        "byte_start": 5918,
        "col": 67,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6043,
        "byte_start": 5990,
        "col": 15,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6084,
        "byte_start": 6043,
        "col": 68,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6174,
        "byte_start": 6115,
        "col": 15,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6215,
        "byte_start": 6174,
        "col": 74,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6288,
        "byte_start": 6246,
        "col": 15,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6329,
        "byte_start": 6288,
        "col": 57,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6403,
        "byte_start": 6360,
        "col": 15,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6444,
        "byte_start": 6403,
        "col": 58,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6521,
        "byte_start": 6475,
        "col": 15,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6562,
        "byte_start": 6521,
        "col": 61,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6638,
        "byte_start": 6593,
        "col": 15,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6679,
        "byte_start": 6638,
        "col": 60,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6751,
        "byte_start": 6710,
        "col": 15,
        "line": 101
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6792,
        "byte_start": 6751,
        "col": 56,
        "line": 101
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6865,
        "byte_start": 6823,
        "col": 15,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6906,
        "byte_start": 6865,
        "col": 57,
        "line": 102
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 6980,
        "byte_start": 6937,
        "col": 15,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7021,
        "byte_start": 6980,
        "col": 58,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7093,
        "byte_start": 7052,
        "col": 15,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7134,
        "byte_start": 7093,
        "col": 56,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7207,
        "byte_start": 7165,
        "col": 15,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7248,
        "byte_start": 7207,
        "col": 57,
        "line": 105
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7321,
        "byte_start": 7279,
        "col": 15,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7362,
        "byte_start": 7321,
        "col": 57,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7436,
        "byte_start": 7393,
        "col": 15,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7477,
        "byte_start": 7436,
        "col": 58,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7556,
        "byte_start": 7508,
        "col": 15,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7597,
        "byte_start": 7556,
        "col": 63,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7685,
        "byte_start": 7628,
        "col": 15,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7726,
        "byte_start": 7685,
        "col": 72,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7819,
        "byte_start": 7757,
        "col": 15,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7860,
        "byte_start": 7819,
        "col": 77,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7933,
        "byte_start": 7891,
        "col": 15,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7974,
        "byte_start": 7933,
        "col": 57,
        "line": 111
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8041,
        "byte_start": 8005,
        "col": 15,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8082,
        "byte_start": 8041,
        "col": 51,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8150,
        "byte_start": 8113,
        "col": 15,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8191,
        "byte_start": 8150,
        "col": 52,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8256,
        "byte_start": 8222,
        "col": 15,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8297,
        "byte_start": 8256,
        "col": 49,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8364,
        "byte_start": 8328,
        "col": 15,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8405,
        "byte_start": 8364,
        "col": 51,
        "line": 115
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8476,
        "byte_start": 8436,
        "col": 15,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8517,
        "byte_start": 8476,
        "col": 55,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8582,
        "byte_start": 8548,
        "col": 15,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8623,
        "byte_start": 8582,
        "col": 49,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8707,
        "byte_start": 8654,
        "col": 15,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8748,
        "byte_start": 8707,
        "col": 68,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8831,
        "byte_start": 8779,
        "col": 15,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8872,
        "byte_start": 8831,
        "col": 67,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 8951,
        "byte_start": 8903,
        "col": 15,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8992,
        "byte_start": 8951,
        "col": 63,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9092,
        "byte_start": 9051,
        "col": 15,
        "line": 124
      }
    },
    {
      "category": "Html",
      "code": "html.picture.text.disallowed",
      "message": "Text not allowed in “picture” in this context.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9167,
        "byte_start": 9126,
        "col": 15,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “br” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9171,
        "byte_start": 9167,
        "col": 56,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9243,
        "byte_start": 9202,
        "col": 15,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9325,
        "byte_start": 9284,
        "col": 15,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9344,
        "byte_start": 9325,
        "col": 56,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9344,
        "byte_start": 9325,
        "col": 56,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9395,
        "byte_start": 9375,
        "col": 15,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9395,
        "byte_start": 9375,
        "col": 15,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9414,
        "byte_start": 9395,
        "col": 35,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.img.disallowed_multiple",
      "message": "Element “img” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9414,
        "byte_start": 9395,
        "col": 35,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9486,
        "byte_start": 9445,
        "col": 15,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9511,
        "byte_start": 9486,
        "col": 56,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9511,
        "byte_start": 9486,
        "col": 56,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9562,
        "byte_start": 9542,
        "col": 15,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9562,
        "byte_start": 9542,
        "col": 15,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.source.disallowed_after_img",
      "message": "Element “source” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9587,
        "byte_start": 9562,
        "col": 35,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9587,
        "byte_start": 9562,
        "col": 35,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9654,
        "byte_start": 9648,
        "col": 15,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9699,
        "byte_start": 9679,
        "col": 46,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9699,
        "byte_start": 9679,
        "col": 46,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9716,
        "byte_start": 9706,
        "col": 73,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9743,
        "byte_start": 9737,
        "col": 15,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9809,
        "byte_start": 9768,
        "col": 46,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9826,
        "byte_start": 9816,
        "col": 94,
        "line": 135
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9872,
        "byte_start": 9847,
        "col": 15,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.picture.child.disallowed",
      "message": "Element “span” not allowed as child of “picture” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 9878,
        "byte_start": 9872,
        "col": 40,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9919,
        "byte_start": 9878,
        "col": 46,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.picture.missing_img",
      "message": "Element “picture” is missing a required instance of child element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 9936,
        "byte_start": 9926,
        "col": 94,
        "line": 136
      }
    },
    {
      "category": "Html",
      "code": "html.source.srcset.invalid",
      "message": "Bad value “data:,b” for attribute “srcset” on element “source”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10000,
        "byte_start": 9975,
        "col": 15,
        "line": 140
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 10027,
        "byte_start": 10000,
        "col": 40,
        "line": 140
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 10027,
        "byte_start": 10000,
        "col": 40,
        "line": 140
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-source-set.html"
}
```
