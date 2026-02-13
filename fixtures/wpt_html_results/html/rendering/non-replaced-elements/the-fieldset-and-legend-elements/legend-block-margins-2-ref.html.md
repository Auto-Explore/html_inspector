# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-2-ref.html

Counts:
- errors: 0
- warnings: 71
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-2-ref.html",
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
  <title>Reference for legend block-axis margins</title>
<style>
.fieldset {
  display: block;
  position: relative;
  width: 40px;
  border: 2px solid blue;
  padding: 4px;
}
.legend {
  display: block;
  position: absolute;
  width: 10px;
  height: 20px;
  padding: 0;
  margin: 0;
  background: grey;
}

.t2 .fieldset { border-top-width: 12px; }
.t3 .fieldset { border-top-width: 12px; }
.t3 .legend { height: 12px; background:white; }
.legend > x { display:block; position:relative; height:6px; background:grey; margin-top:3px; }

div { border: 1px solid;  margin: 0 2px 10px 0; }
c { display:block; height:10px; background: lightgrey; }
f { float: left; }
</style>
</head>
<body>

<f>
<div>
  <span class="fieldset" style="margin-top:9px"><span class="legend" style="top:-11px"></span><c style="margin-top:9px"></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top:19px"><span class="legend" style="top:-11px"></span><c style="margin-top:9px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-12px"></span><c style="margin-top:8px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-20px"></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-20px"></span><c></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top:9px"><span class="legend" style="top:-11px"></span><c style="margin-top:19px"></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top:9px"><span class="legend" style="top:-11px"></span><c style="margin-top:29px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-2px; z-index:1"></span><c style="position:relative; z-index:1"></c></span>
</div>
</f>

<f class=t2>
<div>
  <span class="fieldset" style="margin-top:4px"><span class="legend" style="top:-16px"></span><c style="margin-top:4px"></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top:14px"><span class="legend" style="top:-16px"></span><c style="margin-top:4px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-20px"></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-20px"></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-20px"></span><c></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top:4px"><span class="legend" style="top:-16px"></span><c style="margin-top:14px"></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top:4px"><span class="legend" style="top:-16px"></span><c style="margin-top:24px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="top:-12px; height:16px"></span><c></c></span>
</div>
</f>

<f class=t3>
<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x></x></span><c></c></span>
</div>

<div>
  <span class="fieldset" style="margin-top: 4px"><span class="legend" style="margin-top: -16px"><x style="top:3px"></x></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x></x></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x></x></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x></x></span><c></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x style="top:-3px"></x></span><c style="margin-top:4px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x style="top:-3px"></x></span><c style="margin-top:14px"></c></span>
</div>

<div>
  <span class="fieldset"><span class="legend" style="margin-top: -16px"><x></x></span><c></c></span>
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
        "byte_end": 861,
        "byte_start": 858,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “f” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 861,
        "byte_start": 858,
        "col": 1,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 988,
        "byte_start": 962,
        "col": 95,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 988,
        "byte_start": 962,
        "col": 95,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1135,
        "byte_start": 1109,
        "col": 96,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1135,
        "byte_start": 1109,
        "col": 96,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1258,
        "byte_start": 1232,
        "col": 72,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1258,
        "byte_start": 1232,
        "col": 72,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1358,
        "byte_start": 1355,
        "col": 72,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1358,
        "byte_start": 1355,
        "col": 72,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1458,
        "byte_start": 1455,
        "col": 72,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1458,
        "byte_start": 1455,
        "col": 72,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1605,
        "byte_start": 1578,
        "col": 95,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1605,
        "byte_start": 1578,
        "col": 95,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1752,
        "byte_start": 1725,
        "col": 95,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1752,
        "byte_start": 1725,
        "col": 95,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1899,
        "byte_start": 1859,
        "col": 82,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1899,
        "byte_start": 1859,
        "col": 82,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “f” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1936,
        "byte_start": 1924,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “f” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1936,
        "byte_start": 1924,
        "col": 1,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2063,
        "byte_start": 2037,
        "col": 95,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2063,
        "byte_start": 2037,
        "col": 95,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2210,
        "byte_start": 2184,
        "col": 96,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2210,
        "byte_start": 2184,
        "col": 96,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2310,
        "byte_start": 2307,
        "col": 72,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2310,
        "byte_start": 2307,
        "col": 72,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2410,
        "byte_start": 2407,
        "col": 72,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2410,
        "byte_start": 2407,
        "col": 72,
        "line": 87
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2510,
        "byte_start": 2507,
        "col": 72,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2510,
        "byte_start": 2507,
        "col": 72,
        "line": 91
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2657,
        "byte_start": 2630,
        "col": 95,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2657,
        "byte_start": 2630,
        "col": 95,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2804,
        "byte_start": 2777,
        "col": 95,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2804,
        "byte_start": 2777,
        "col": 95,
        "line": 99
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2917,
        "byte_start": 2914,
        "col": 85,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2917,
        "byte_start": 2914,
        "col": 85,
        "line": 103
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “f” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2954,
        "byte_start": 2942,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “f” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 2954,
        "byte_start": 2942,
        "col": 1,
        "line": 107
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3036,
        "byte_start": 3033,
        "col": 73,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3036,
        "byte_start": 3033,
        "col": 73,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3050,
        "byte_start": 3047,
        "col": 87,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3050,
        "byte_start": 3047,
        "col": 87,
        "line": 109
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3191,
        "byte_start": 3172,
        "col": 97,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3191,
        "byte_start": 3172,
        "col": 97,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3205,
        "byte_start": 3202,
        "col": 127,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3205,
        "byte_start": 3202,
        "col": 127,
        "line": 113
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3306,
        "byte_start": 3303,
        "col": 73,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3306,
        "byte_start": 3303,
        "col": 73,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3320,
        "byte_start": 3317,
        "col": 87,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3320,
        "byte_start": 3317,
        "col": 87,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3421,
        "byte_start": 3418,
        "col": 73,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3421,
        "byte_start": 3418,
        "col": 73,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3435,
        "byte_start": 3432,
        "col": 87,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3435,
        "byte_start": 3432,
        "col": 87,
        "line": 121
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3536,
        "byte_start": 3533,
        "col": 73,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3536,
        "byte_start": 3533,
        "col": 73,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3550,
        "byte_start": 3547,
        "col": 87,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3550,
        "byte_start": 3547,
        "col": 87,
        "line": 125
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3668,
        "byte_start": 3648,
        "col": 73,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3668,
        "byte_start": 3648,
        "col": 73,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3705,
        "byte_start": 3679,
        "col": 104,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3705,
        "byte_start": 3679,
        "col": 104,
        "line": 129
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3823,
        "byte_start": 3803,
        "col": 73,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3823,
        "byte_start": 3803,
        "col": 73,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3861,
        "byte_start": 3834,
        "col": 104,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3861,
        "byte_start": 3834,
        "col": 104,
        "line": 133
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “x” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3962,
        "byte_start": 3959,
        "col": 73,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “x” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3962,
        "byte_start": 3959,
        "col": 73,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “c” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 3976,
        "byte_start": 3973,
        "col": 87,
        "line": 137
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “c” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 3976,
        "byte_start": 3973,
        "col": 87,
        "line": 137
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-block-margins-2-ref.html"
}
```
