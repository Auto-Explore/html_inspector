# html/semantics/embedded-content/image-maps/image-map-processing-model/hash-name-reference-test-data.html

Counts:
- errors: 43
- warnings: 33
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/image-maps/image-map-processing-model/hash-name-reference-test-data.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE {{GET[doctype]}}>
<!-- This file should be polyglot -->
<html xmlns="http://www.w3.org/1999/xhtml">
 <head>
  <meta charset="utf-8"/>
  <title>Test data for hash name reference</title>
  <style>
   body { margin: 0 }
   img, object { height: 1px; display:block }
  </style>
 </head>
 <body>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="no-hash-name"/>
 <object data="/images/threecolors.png" usemap="no-hash-name"></object>
 <map name="no-hash-name">
  <area shape="rect" coords="0,0,99,50" href="#area-no-hash-name"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="no-hash-id"/>
 <object data="/images/threecolors.png" usemap="no-hash-id"></object>
 <map id="no-hash-id">
  <area shape="rect" coords="0,0,99,50" href="#area-no-hash-id"/>
 </map>
</div>

<div data-expect="area-hash-name">
 <img src="/images/threecolors.png" usemap="#hash-name"/>
 <object data="/images/threecolors.png" usemap="#hash-name"></object>
 <map name="hash-name">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-name"/>
 </map>
</div>

<div data-expect="area-hash-id">
 <img src="/images/threecolors.png" usemap="#hash-id"/>
 <object data="/images/threecolors.png" usemap="#hash-id"></object>
 <map id="hash-id">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-id"/>
 </map>
</div>

<div data-expect="area-non-map-with-this-name">
 <img src="/images/threecolors.png" usemap="#non-map-with-this-name" name="non-map-with-this-name"/>
 <object data="/images/threecolors.png" usemap="#non-map-with-this-name"></object>
 <map name="non-map-with-this-name">
  <area shape="rect" coords="0,0,99,50" href="#area-non-map-with-this-name"/>
 </map>
</div>

<div data-expect="area-non-map-with-this-id">
 <img src="/images/threecolors.png" usemap="#non-map-with-this-id" id="non-map-with-this-id"/>
 <object data="/images/threecolors.png" usemap="#non-map-with-this-id"></object>
 <map id="non-map-with-this-id">
  <area shape="rect" coords="0,0,99,50" href="#area-non-map-with-this-id"/>
 </map>
</div>

<div data-expect="area-two-maps-with-this-name-1">
 <img src="/images/threecolors.png" usemap="#two-maps-with-this-name"/>
 <object data="/images/threecolors.png" usemap="#two-maps-with-this-name"></object>
 <map name="two-maps-with-this-name">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-name-1"/>
 </map>
 <map name="two-maps-with-this-name">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-name-2"/>
 </map>
</div>

<div data-expect="area-two-maps-with-this-id-1">
 <img src="/images/threecolors.png" usemap="#two-maps-with-this-id"/>
 <object data="/images/threecolors.png" usemap="#two-maps-with-this-id"></object>
 <map id="two-maps-with-this-id">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-id-1"/>
 </map>
 <map id="two-maps-with-this-id">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-id-2"/>
 </map>
</div>

<div data-expect="area-two-maps-with-this-name-or-id-1">
 <img src="/images/threecolors.png" usemap="#two-maps-with-this-name-or-id"/>
 <object data="/images/threecolors.png" usemap="#two-maps-with-this-name-or-id"></object>
 <map name="two-maps-with-this-name-or-id">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-name-or-id-1"/>
 </map>
 <map id="two-maps-with-this-name-or-id">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-name-or-id-2"/>
 </map>
</div>

<div data-expect="area-two-maps-with-this-id-or-name-1">
 <img src="/images/threecolors.png" usemap="#two-maps-with-this-id-or-name"/>
 <object data="/images/threecolors.png" usemap="#two-maps-with-this-id-or-name"></object>
 <map id="two-maps-with-this-id-or-name">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-id-or-name-1"/>
 </map>
 <map name="two-maps-with-this-id-or-name">
  <area shape="rect" coords="0,0,99,50" href="#area-two-maps-with-this-id-or-name-2"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="hash-last#"/>
 <object data="/images/threecolors.png" usemap="hash-last#"></object>
 <map name="hash-last" id="hash-last">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-last-no-hash-in-map-name-and-id"/>
 </map>
 <map name="hash-last#" id="hash-last#">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-last-with-hash-in-map-name-and-id"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap=""/>
 <object data="/images/threecolors.png" usemap=""></object>
 <map name="" id="">
  <area shape="rect" coords="0,0,99,50" href="#area-empty-usemap-empty-map-name-and-id"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="#"/>
 <object data="/images/threecolors.png" usemap="#"></object>
 <map name="" id="">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-usemap-empty-name-and-id"/>
 </map>
</div>

<div data-expect="area-hash-space-usemap-space-map-name">
 <img src="/images/threecolors.png" usemap="# "/>
 <object data="/images/threecolors.png" usemap="# "></object>
 <map name=" ">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-space-usemap-space-map-name"/>
 </map>
</div>

<div data-expect="area-hash-LF-usemap-LF-map-id">
 <img src="/images/threecolors.png" usemap="#&#x0A;"/>
 <object data="/images/threecolors.png" usemap="#&#x0A;"></object>
 <map id="&#x0A;">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-LF-usemap-LF-map-id"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="#percent-escape-name-%41"/>
 <object data="/images/threecolors.png" usemap="#percent-escape-name-%41"></object>
 <map name="percent-escape-name-A">
  <area shape="rect" coords="0,0,99,50" href="#area-percent-escape-name-A"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="#percent-escape-id-%41"/>
 <object data="/images/threecolors.png" usemap="#percent-escape-id-%41"></object>
 <map id="percent-escape-id-A">
  <area shape="rect" coords="0,0,99,50" href="#area-percent-escape-id-A"/>
 </map>
</div>

<div data-expect="area-percent-escape-name-B">
 <img src="/images/threecolors.png" usemap="#percent-escape-name-%42"/>
 <object data="/images/threecolors.png" usemap="#percent-escape-name-%42"></object>
 <map name="percent-escape-name-%42">
  <area shape="rect" coords="0,0,99,50" href="#area-percent-escape-name-B"/>
 </map>
</div>

<div data-expect="area-percent-escape-id-B">
 <img src="/images/threecolors.png" usemap="#percent-escape-id-%42"/>
 <object data="/images/threecolors.png" usemap="#percent-escape-id-%42"></object>
 <map id="percent-escape-id-%42">
  <area shape="rect" coords="0,0,99,50" href="#area-percent-escape-id-B"/>
 </map>
</div>

<div data-expect="area-hash-space-name">
 <img src="/images/threecolors.png" usemap="# hash-space-name"/>
 <object data="/images/threecolors.png" usemap="# hash-space-name"></object>
 <map name=" hash-space-name">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-space-name"/>
 </map>
</div>

<div data-expect="area-hash-space-id">
 <img src="/images/threecolors.png" usemap="# hash-space-id"/>
 <object data="/images/threecolors.png" usemap="# hash-space-id"></object>
 <map id=" hash-space-id">
  <area shape="rect" coords="0,0,99,50" href="#area-hash-space-id"/>
 </map>
</div>

<div data-expect="area-space-before-hash-name">
 <img src="/images/threecolors.png" usemap=" #space-before-hash-name"/>
 <object data="/images/threecolors.png" usemap=" #space-before-hash-name"></object>
 <map name="space-before-hash-name">
  <area shape="rect" coords="0,0,99,50" href="#area-space-before-hash-name"/>
 </map>
</div>

<div data-expect="area-space-before-hash-id">
 <img src="/images/threecolors.png" usemap=" #space-before-hash-id"/>
 <object data="/images/threecolors.png" usemap=" #space-before-hash-id"></object>
 <map id="space-before-hash-id">
  <area shape="rect" coords="0,0,99,50" href="#area-space-before-hash-id"/>
 </map>
</div>

<div data-expect="area-garbage-before-hash-name">
 <img src="/images/threecolors.png" usemap="http://example.org/#garbage-before-hash-name"/>
 <object data="/images/threecolors.png" usemap="http://example.org/#garbage-before-hash-name"></object>
 <map name="garbage-before-hash-name">
  <area shape="rect" coords="0,0,99,50" href="#area-garbage-before-hash-name"/>
 </map>
</div>

<div data-expect="area-garbage-before-hash-id">
 <img src="/images/threecolors.png" usemap="http://example.org/#garbage-before-hash-id"/>
 <object data="/images/threecolors.png" usemap="http://example.org/#garbage-before-hash-id"></object>
 <map id="garbage-before-hash-id">
  <area shape="rect" coords="0,0,99,50" href="#area-garbage-before-hash-id"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="#no-such-map"/>
 <object data="/images/threecolors.png" usemap="#no-such-map"></object>
 <map>
  <area shape="rect" coords="0,0,99,50" href="#area-no-such-map"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="#different-CASE-name"/>
 <object data="/images/threecolors.png" usemap="#different-CASE-name"></object>
 <map name="different-case-name">
  <area shape="rect" coords="0,0,99,50" href="#area-different-case-name"/>
 </map>
</div>

<div data-expect="no match">
 <img src="/images/threecolors.png" usemap="#different-CASE-id"/>
 <object data="/images/threecolors.png" usemap="#different-CASE-id"></object>
 <map id="different-case-id">
  <area shape="rect" coords="0,0,99,50" href="#area-different-case-id"/>
 </map>
</div>

 </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “no-hash-name” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 390,
        "byte_start": 332,
        "col": 2,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 390,
        "byte_start": 332,
        "col": 2,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “no-hash-name” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 453,
        "byte_start": 392,
        "col": 2,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “no-hash-id” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 660,
        "byte_start": 604,
        "col": 2,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 660,
        "byte_start": 604,
        "col": 2,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “no-hash-id” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 721,
        "byte_start": 662,
        "col": 2,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 928,
        "byte_start": 872,
        "col": 2,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1192,
        "byte_start": 1138,
        "col": 2,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1508,
        "byte_start": 1409,
        "col": 2,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1863,
        "byte_start": 1770,
        "col": 2,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “non-map-with-this-id”.",
      "severity": "Error",
      "span": {
        "byte_end": 1977,
        "byte_start": 1946,
        "col": 2,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2192,
        "byte_start": 2122,
        "col": 2,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2657,
        "byte_start": 2589,
        "col": 2,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “two-maps-with-this-id”.",
      "severity": "Error",
      "span": {
        "byte_end": 2894,
        "byte_start": 2862,
        "col": 2,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3124,
        "byte_start": 3048,
        "col": 2,
        "line": 85
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 3633,
        "byte_start": 3557,
        "col": 2,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “hash-last#” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 4094,
        "byte_start": 4038,
        "col": 2,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4094,
        "byte_start": 4038,
        "col": 2,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “hash-last#” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 4155,
        "byte_start": 4096,
        "col": 2,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 4531,
        "byte_start": 4485,
        "col": 2,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4531,
        "byte_start": 4485,
        "col": 2,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 4582,
        "byte_start": 4533,
        "col": 2,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “” for attribute “id” on element “map”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4612,
        "byte_start": 4593,
        "col": 2,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “#” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 4796,
        "byte_start": 4749,
        "col": 2,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 4796,
        "byte_start": 4749,
        "col": 2,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “#” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 4848,
        "byte_start": 4798,
        "col": 2,
        "line": 127
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “” for attribute “id” on element “map”.",
      "severity": "Warning",
      "span": {
        "byte_end": 4878,
        "byte_start": 4859,
        "col": 2,
        "line": 128
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5087,
        "byte_start": 5039,
        "col": 2,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5374,
        "byte_start": 5321,
        "col": 2,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “\n” for attribute “id” on element “map”.",
      "severity": "Warning",
      "span": {
        "byte_end": 5460,
        "byte_start": 5443,
        "col": 2,
        "line": 144
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5657,
        "byte_start": 5587,
        "col": 2,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 5969,
        "byte_start": 5901,
        "col": 2,
        "line": 158
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6293,
        "byte_start": 6223,
        "col": 2,
        "line": 166
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6623,
        "byte_start": 6555,
        "col": 2,
        "line": 174
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 6936,
        "byte_start": 6873,
        "col": 2,
        "line": 182
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7233,
        "byte_start": 7172,
        "col": 2,
        "line": 190
      }
    },
    {
      "category": "Html",
      "code": "html.id.invalid",
      "message": "Bad value “ hash-space-id” for attribute “id” on element “map”.",
      "severity": "Warning",
      "span": {
        "byte_end": 7335,
        "byte_start": 7310,
        "col": 2,
        "line": 192
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “ #space-before-hash-name” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 7540,
        "byte_start": 7470,
        "col": 2,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7540,
        "byte_start": 7470,
        "col": 2,
        "line": 198
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “ #space-before-hash-name” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 7615,
        "byte_start": 7542,
        "col": 2,
        "line": 199
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “ #space-before-hash-id” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 7871,
        "byte_start": 7803,
        "col": 2,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 7871,
        "byte_start": 7803,
        "col": 2,
        "line": 206
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “ #space-before-hash-id” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 7944,
        "byte_start": 7873,
        "col": 2,
        "line": 207
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “http://example.org/#garbage-before-hash-name” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 8220,
        "byte_start": 8130,
        "col": 2,
        "line": 214
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8220,
        "byte_start": 8130,
        "col": 2,
        "line": 214
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “http://example.org/#garbage-before-hash-name” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 8315,
        "byte_start": 8222,
        "col": 2,
        "line": 215
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “http://example.org/#garbage-before-hash-id” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 8597,
        "byte_start": 8509,
        "col": 2,
        "line": 222
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8597,
        "byte_start": 8509,
        "col": 2,
        "line": 222
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.bad_value",
      "message": "Bad value “http://example.org/#garbage-before-hash-id” for attribute “usemap” on element “object”.",
      "severity": "Error",
      "span": {
        "byte_end": 8690,
        "byte_start": 8599,
        "col": 2,
        "line": 223
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 8917,
        "byte_start": 8859,
        "col": 2,
        "line": 230
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9176,
        "byte_start": 9110,
        "col": 2,
        "line": 238
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 9476,
        "byte_start": 9412,
        "col": 2,
        "line": 246
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “hash-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 1192,
        "byte_start": 1138,
        "col": 2,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “hash-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 1251,
        "byte_start": 1194,
        "col": 2,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “non-map-with-this-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 1863,
        "byte_start": 1770,
        "col": 2,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “non-map-with-this-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 1935,
        "byte_start": 1865,
        "col": 2,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “two-maps-with-this-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 2657,
        "byte_start": 2589,
        "col": 2,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “two-maps-with-this-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 2730,
        "byte_start": 2659,
        "col": 2,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “\n”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 5374,
        "byte_start": 5321,
        "col": 2,
        "line": 142
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “\n”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 5432,
        "byte_start": 5376,
        "col": 2,
        "line": 143
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “percent-escape-name-%41”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 5657,
        "byte_start": 5587,
        "col": 2,
        "line": 150
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “percent-escape-name-%41”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 5732,
        "byte_start": 5659,
        "col": 2,
        "line": 151
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “percent-escape-id-%41”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 5969,
        "byte_start": 5901,
        "col": 2,
        "line": 158
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “percent-escape-id-%41”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 6042,
        "byte_start": 5971,
        "col": 2,
        "line": 159
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “percent-escape-id-%42”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 6623,
        "byte_start": 6555,
        "col": 2,
        "line": 174
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “percent-escape-id-%42”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 6696,
        "byte_start": 6625,
        "col": 2,
        "line": 175
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “ hash-space-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 7233,
        "byte_start": 7172,
        "col": 2,
        "line": 190
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “ hash-space-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 7299,
        "byte_start": 7235,
        "col": 2,
        "line": 191
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “no-such-map”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 8917,
        "byte_start": 8859,
        "col": 2,
        "line": 230
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “no-such-map”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 8980,
        "byte_start": 8919,
        "col": 2,
        "line": 231
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “different-CASE-name”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 9176,
        "byte_start": 9110,
        "col": 2,
        "line": 238
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “different-CASE-name”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 9247,
        "byte_start": 9178,
        "col": 2,
        "line": 239
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “different-CASE-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 9476,
        "byte_start": 9412,
        "col": 2,
        "line": 246
      }
    },
    {
      "category": "Html",
      "code": "html.object.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “different-CASE-id”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 9545,
        "byte_start": 9478,
        "col": 2,
        "line": 247
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
  "source_name": "html/semantics/embedded-content/image-maps/image-map-processing-model/hash-name-reference-test-data.html"
}
```
