# html/rendering/sections/headingoffset-and-headingreset.html

Counts:
- errors: 0
- warnings: 48
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/sections/headingoffset-and-headingreset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<link rel="help" href="https://github.com/whatwg/html/pull/11086" />
<link rel="match" href="headingoffset-and-headingreset-ref.html">
<title>Test :heading default styles</title>
<style>
  :heading(1)::after { content: 'h1' }
  :heading(2)::after { content: 'h2' }
  :heading(3)::after { content: 'h3' }
  :heading(4)::after { content: 'h4' }
  :heading(5)::after { content: 'h5' }
  :heading(6)::after { content: 'h6' }
  :heading(7)::after { content: 'h7' }
  :heading(8)::after { content: 'h8' }
  :heading(9)::after { content: 'h9' }
</style>
<div>
  <h1></h1>
  <h2></h2>
  <h3></h3>
  <h4></h4>
  <h5></h5>
  <h6></h6>
  <h6 headingoffset=1></h6>
  <h6 headingoffset=2></h6>
  <h6 headingoffset=3></h6>
</div>
<hr>
<div headingoffset=2>
  <h1></h1>
  <h2></h2>
  <h3></h3>
  <h4></h4>
  <h5></h5>
  <h6></h6>
  <h6 headingoffset=1></h6>
  <h6 headingoffset=2></h6>
  <h6 headingoffset=3></h6>
</div>
<hr>
<div headingoffset=4>
  <h1></h1>
  <h2></h2>
  <h3></h3>
  <h4></h4>
  <h5></h5>
  <h6></h6>
  <h6 headingoffset=1></h6>
  <h6 headingoffset=2></h6>
  <h6 headingoffset=3></h6>
</div>
<hr>
<div headingoffset=6>
  <h1></h1>
  <h2></h2>
  <h3></h3>
  <h4></h4>
  <h5></h5>
  <h6></h6>
  <h6 headingoffset=1></h6>
  <h6 headingoffset=2></h6>
  <h6 headingoffset=3></h6>
</div>
<hr>
<div headingoffset=8>
  <h1></h1>
  <h2></h2>
  <h3></h3>
  <h4></h4>
  <h5></h5>
  <h6></h6>
  <h6 headingoffset=1></h6>
  <h6 headingoffset=2></h6>
  <h6 headingoffset=3></h6>
</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 95,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 95,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 660,
        "byte_start": 655,
        "col": 7,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 672,
        "byte_start": 667,
        "col": 7,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 684,
        "byte_start": 679,
        "col": 7,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 696,
        "byte_start": 691,
        "col": 7,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 708,
        "byte_start": 703,
        "col": 7,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 720,
        "byte_start": 715,
        "col": 7,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 748,
        "byte_start": 743,
        "col": 23,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 776,
        "byte_start": 771,
        "col": 23,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 804,
        "byte_start": 799,
        "col": 23,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 850,
        "byte_start": 845,
        "col": 7,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 862,
        "byte_start": 857,
        "col": 7,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 874,
        "byte_start": 869,
        "col": 7,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 886,
        "byte_start": 881,
        "col": 7,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 898,
        "byte_start": 893,
        "col": 7,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 910,
        "byte_start": 905,
        "col": 7,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 938,
        "byte_start": 933,
        "col": 23,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 966,
        "byte_start": 961,
        "col": 23,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 994,
        "byte_start": 989,
        "col": 23,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1040,
        "byte_start": 1035,
        "col": 7,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1052,
        "byte_start": 1047,
        "col": 7,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1064,
        "byte_start": 1059,
        "col": 7,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1076,
        "byte_start": 1071,
        "col": 7,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1088,
        "byte_start": 1083,
        "col": 7,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1100,
        "byte_start": 1095,
        "col": 7,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1128,
        "byte_start": 1123,
        "col": 23,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1156,
        "byte_start": 1151,
        "col": 23,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1184,
        "byte_start": 1179,
        "col": 23,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1230,
        "byte_start": 1225,
        "col": 7,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1242,
        "byte_start": 1237,
        "col": 7,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1254,
        "byte_start": 1249,
        "col": 7,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1266,
        "byte_start": 1261,
        "col": 7,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1278,
        "byte_start": 1273,
        "col": 7,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1290,
        "byte_start": 1285,
        "col": 7,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1318,
        "byte_start": 1313,
        "col": 23,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1346,
        "byte_start": 1341,
        "col": 23,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1374,
        "byte_start": 1369,
        "col": 23,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1420,
        "byte_start": 1415,
        "col": 7,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1432,
        "byte_start": 1427,
        "col": 7,
        "line": 67
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1444,
        "byte_start": 1439,
        "col": 7,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1456,
        "byte_start": 1451,
        "col": 7,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1468,
        "byte_start": 1463,
        "col": 7,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1480,
        "byte_start": 1475,
        "col": 7,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1508,
        "byte_start": 1503,
        "col": 23,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1536,
        "byte_start": 1531,
        "col": 23,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1564,
        "byte_start": 1559,
        "col": 23,
        "line": 74
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
  "source_name": "html/rendering/sections/headingoffset-and-headingreset.html"
}
```
