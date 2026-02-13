# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-2.html

Counts:
- errors: 0
- warnings: 55
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<!--
     Any copyright is dedicated to the Public Domain.
     http://creativecommons.org/publicdomain/zero/1.0/
-->
<html><head>
  <meta charset="utf-8">
  <title>legend block-axis margins</title>
  <link rel="match" href="legend-block-margins-2-ref.html">
<style>
fieldset {
  width: 40px;
  border: 2px solid blue;
  padding: 4px;
  margin: 0;
}
legend {
  width: 10px;
  height: 20px;
  padding: 0;
  background: grey;

}

.t2 fieldset { border-top-width: 12px; }
.t3 fieldset { border-top-width: 12px; }
.t3 legend { height: 6px; }

div { border: 1px solid;  margin: 0 2px 10px 0; }
c { display:block; height:10px; background: lightgrey; }
f { float: left; }
</style>
</head>
<body>

<f>
<div>
  <fieldset><legend></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: 10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -20px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -30px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: 10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: 20px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: -20px"></legend><c></c></fieldset>
</div>
</f>

<f class=t2>
<div>
  <fieldset><legend></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: 10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -20px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -30px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: 10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: 20px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: -20px"></legend><c></c></fieldset>
</div>
</f>

<f class=t3>
<div>
  <fieldset><legend></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: 10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -20px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-top: -30px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: 10px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: 20px"></legend><c></c></fieldset>
</div>

<div>
  <fieldset><legend style="margin-bottom: -20px"></legend><c></c></fieldset>
</div>
</f>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “f” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 709,
        "byte_start": 706,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “f” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 709,
        "byte_start": 706,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 748,
        "byte_start": 745,
        "col": 30,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 748,
        "byte_start": 745,
        "col": 30,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 835,
        "byte_start": 832,
        "col": 55,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 835,
        "byte_start": 832,
        "col": 55,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 923,
        "byte_start": 920,
        "col": 56,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 923,
        "byte_start": 920,
        "col": 56,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1011,
        "byte_start": 1008,
        "col": 56,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1011,
        "byte_start": 1008,
        "col": 56,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1099,
        "byte_start": 1096,
        "col": 56,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1099,
        "byte_start": 1096,
        "col": 56,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1189,
        "byte_start": 1186,
        "col": 58,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1189,
        "byte_start": 1186,
        "col": 58,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1279,
        "byte_start": 1276,
        "col": 58,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1279,
        "byte_start": 1276,
        "col": 58,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1370,
        "byte_start": 1367,
        "col": 59,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1370,
        "byte_start": 1367,
        "col": 59,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “f” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1411,
        "byte_start": 1399,
        "col": 1,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “f” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1411,
        "byte_start": 1399,
        "col": 1,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1450,
        "byte_start": 1447,
        "col": 30,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1450,
        "byte_start": 1447,
        "col": 30,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1537,
        "byte_start": 1534,
        "col": 55,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1537,
        "byte_start": 1534,
        "col": 55,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1625,
        "byte_start": 1622,
        "col": 56,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1625,
        "byte_start": 1622,
        "col": 56,
        "line": 80
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1713,
        "byte_start": 1710,
        "col": 56,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1713,
        "byte_start": 1710,
        "col": 56,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1801,
        "byte_start": 1798,
        "col": 56,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1801,
        "byte_start": 1798,
        "col": 56,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1891,
        "byte_start": 1888,
        "col": 58,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1891,
        "byte_start": 1888,
        "col": 58,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1981,
        "byte_start": 1978,
        "col": 58,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1981,
        "byte_start": 1978,
        "col": 58,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2072,
        "byte_start": 2069,
        "col": 59,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2072,
        "byte_start": 2069,
        "col": 59,
        "line": 100
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “f” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2113,
        "byte_start": 2101,
        "col": 1,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “f” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2113,
        "byte_start": 2101,
        "col": 1,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2152,
        "byte_start": 2149,
        "col": 30,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2152,
        "byte_start": 2149,
        "col": 30,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2239,
        "byte_start": 2236,
        "col": 55,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2239,
        "byte_start": 2236,
        "col": 55,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2327,
        "byte_start": 2324,
        "col": 56,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2327,
        "byte_start": 2324,
        "col": 56,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2415,
        "byte_start": 2412,
        "col": 56,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2415,
        "byte_start": 2412,
        "col": 56,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2503,
        "byte_start": 2500,
        "col": 56,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2503,
        "byte_start": 2500,
        "col": 56,
        "line": 122
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2593,
        "byte_start": 2590,
        "col": 58,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2593,
        "byte_start": 2590,
        "col": 58,
        "line": 126
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2683,
        "byte_start": 2680,
        "col": 58,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2683,
        "byte_start": 2680,
        "col": 58,
        "line": 130
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “fieldset” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2774,
        "byte_start": 2771,
        "col": 59,
        "line": 134
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2774,
        "byte_start": 2771,
        "col": 59,
        "line": 134
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-2.html"
}
```
