# html/semantics/selectors/pseudo-classes/inrange-outofrange.html

Counts:
- errors: 0
- warnings: 19
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/inrange-outofrange.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:in-range, :out-of-range)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id="link1">
<link rel="author" title="Chris Rebert" href="http://chrisrebert.com" id="link2">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#selector-in-range" id="link3">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#selector-out-of-range" id="link4">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<div id="log"></div>
<input type=number value=0 min=0 max=10 id=number1>
<input type=number value=0 min=0 max=10 id=number2 disabled>
<input type=number value=0 min=1 max=10 id=number3>
<input type=number value=11 min=0 max=10 id=number4>
<input type=number value=0 min=0 max=10 id=number5 readonly>

<input type="date" min="2005-10-10" max="2020-10-10" value="2010-10-10" id="datein">
<input type="date" min="2010-10-10" max="2020-10-10" value="2005-10-10" id="dateunder">
<input type="date" min="2010-10-10" max="2020-10-10" value="2030-10-10" id="dateover">

<input type="time" min="01:00:00" max="05:00:00" value="02:00:00" id="timein">
<input type="time" min="02:00:00" max="05:00:00" value="01:00:00" id="timeunder">
<input type="time" min="02:00:00" max="05:00:00" value="07:00:00" id="timeover">

<input type="week" min="2016-W05" max="2016-W10" value="2016-W07" id="weekin">
<input type="week" min="2016-W05" max="2016-W10" value="2016-W02" id="weekunder">
<input type="week" min="2016-W05" max="2016-W10" value="2016-W26" id="weekover">

<input type="month" min="2000-04" max="2000-09" value="2000-06" id="monthin">
<input type="month" min="2000-04" max="2000-09" value="2000-02" id="monthunder">
<input type="month" min="2000-04" max="2000-09" value="2000-11" id="monthover">

<input type="datetime-local" min="2008-03-12T23:59:59" max="2015-02-13T23:59:59" value="2012-11-28T23:59:59" id="datetimelocalin">
<input type="datetime-local" min="2008-03-12T23:59:59" max="2015-02-13T23:59:59" value="2008-03-01T23:59:59" id="datetimelocalunder">
<input type="datetime-local" min="2008-03-12T23:59:59" max="2015-02-13T23:59:59" value="2016-01-01T23:59:59" id="datetimelocalover">

<!-- None of the following have range limitations since they have neither min nor max attributes -->
<input type="number" value="0" id="numbernolimit">
<input type="date" value="2010-10-10" id="datenolimit">
<input type="time" value="02:00:00" id="timenolimit">
<input type="week" value="2016-W07" id="weeknolimit">
<input type="month" value="2000-06" id="monthnolimit">
<input type="datetime-local" value="2012-11-28T23:59:59" id="datetimelocalnolimit">

<!-- range inputs have default minimum of 0 and default maximum of 100 -->
<input type="range" value="50" id="range0">

<!-- range input's value gets immediately clamped to the nearest boundary point -->
<input type="range" min="2" max="7" value="5" id="range1">
<input type="range" min="2" max="7" value="1" id="range2">
<input type="range" min="2" max="7" value="9" id="range3">

<!-- None of the following input types can have range limitations -->
<input min="1" value="0" type="text">
<input min="1" value="0" type="search">
<input min="1" value="0" type="url">
<input min="1" value="0" type="tel">
<input min="1" value="0" type="email">
<input min="1" value="0" type="password">
<input min="1" value="#000000" type="color">
<input min="1" value="0" type="checkbox">
<input min="1" value="0" type="radio">
<input min="1" value="0" type="file">
<input min="1" value="0" type="submit">
<input min="1" value="0" type="image">
<!-- The following types are also barred from constraint validation -->
<input min="1" value="0" type="hidden">
<input min="1" value="0" type="button">
<input min="1" value="0" type="reset">

<script>
  testSelectorIdsMatch(":in-range", ["number1", "datein", "timein", "weekin", "monthin", "datetimelocalin", "range0", "range1", "range2", "range3"], "':in-range' matches all elements that are candidates for constraint validation, have range limitations, and that are neither suffering from an underflow nor suffering from an overflow");

  testSelectorIdsMatch(":out-of-range", ["number3", "number4", "dateunder", "dateover", "timeunder", "timeover", "weekunder", "weekover", "monthunder", "monthover", "datetimelocalunder", "datetimelocalover"], "':out-of-range' matches all elements that are candidates for constraint validation, have range limitations, and that are either suffering from an underflow or suffering from an overflow");

  document.getElementById("number1").value = -10;
  testSelectorIdsMatch(":in-range", ["datein", "timein", "weekin", "monthin", "datetimelocalin", "range0", "range1", "range2", "range3"], "':in-range' update number1's value < min");
  testSelectorIdsMatch(":out-of-range", ["number1", "number3", "number4", "dateunder", "dateover", "timeunder", "timeover", "weekunder", "weekover", "monthunder", "monthover", "datetimelocalunder", "datetimelocalover"], "':out-of-range' update number1's value < min");

  document.getElementById("number3").min = 0;
  testSelectorIdsMatch(":in-range", ["number3", "datein", "timein", "weekin", "monthin", "datetimelocalin", "range0", "range1", "range2", "range3"], "':in-range' update number3's min < value");
  testSelectorIdsMatch(":out-of-range", ["number1", "number4", "dateunder", "dateover", "timeunder", "timeover", "weekunder", "weekover", "monthunder", "monthover", "datetimelocalunder", "datetimelocalover"], "':out-of-range' update number3's min < value");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3229,
        "byte_start": 3192,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3269,
        "byte_start": 3230,
        "col": 1,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3306,
        "byte_start": 3270,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “0” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3306,
        "byte_start": 3270,
        "col": 1,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3343,
        "byte_start": 3307,
        "col": 1,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3382,
        "byte_start": 3344,
        "col": 1,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3424,
        "byte_start": 3383,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3469,
        "byte_start": 3425,
        "col": 1,
        "line": 61
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3511,
        "byte_start": 3470,
        "col": 1,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3550,
        "byte_start": 3512,
        "col": 1,
        "line": 63
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3588,
        "byte_start": 3551,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.input.file.value.disallowed",
      "message": "Attribute “value” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3588,
        "byte_start": 3551,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3628,
        "byte_start": 3589,
        "col": 1,
        "line": 65
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3667,
        "byte_start": 3629,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 3667,
        "byte_start": 3629,
        "col": 1,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3779,
        "byte_start": 3740,
        "col": 1,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3819,
        "byte_start": 3780,
        "col": 1,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.input.min.disallowed_for_type",
      "message": "Attribute “min” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 3858,
        "byte_start": 3820,
        "col": 1,
        "line": 70
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
  "source_name": "html/semantics/selectors/pseudo-classes/inrange-outofrange.html"
}
```
