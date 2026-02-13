# html/rendering/non-replaced-elements/form-controls/resets.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/resets.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>default style resets</title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/rendering/support/test-ua-stylesheet.js"></script>
<style>
/* Have some non-initial values on the parent so we can tell the difference whether the UA stylesheet uses 'initial' keyword. */
#tests, #refs {
  letter-spacing: 5px;
  word-spacing: 5px;
  line-height: 5px;
  text-transform: uppercase;
  text-indent: 5px;
  text-shadow: 0 0 5px transparent;
  text-align: right;
}
</style>
<style>
/* Specify this bogus namespace, so the rules in this stylesheet only apply to the `fakeClone`d elements in #refs, not the HTML elements in #tests. */
@namespace url(urn:not-html);

input, select, button, textarea {
  letter-spacing: initial;
  word-spacing: initial;
  line-height: initial;
  text-transform: initial;
  text-indent: initial;
  text-shadow: initial;
}
input, select, textarea {
  text-align: initial;
}
input[type=reset i], input[type=button i], input[type=submit i], button {
  text-align: center;
}
input[type=radio i], input[type=checkbox i], input[type=reset i], input[type=button i],
input[type=submit i], input[type=color i], input[type=search i], select, button {
  box-sizing: border-box;
}
input, button {
  display: inline-block;
}
input:not([type=image i], [type=range i], [type=checkbox i], [type=radio i]) {
  overflow: clip;
  overflow-clip-margin: 0;
}
/* in spec prose: */ select, textarea, meter, progress {
  display: inline-block;
}
input[type=hidden i] { display: none !important; }
marquee {
  overflow: hidden;
  text-align: initial;
}
table { display: table; box-sizing: border-box; }
caption { display: table-caption; }
colgroup, colgroup[hidden] { display: table-column-group; }
col, col[hidden] { display: table-column; }
thead, thead[hidden] { display: table-header-group; }
tbody, tbody[hidden] { display: table-row-group; }
tfoot, tfoot[hidden] { display: table-footer-group; }
tr, tr[hidden] { display: table-row; }
td, th { display: table-cell; }
table {
  text-indent: initial;
}
</style>

<div id="tests">
 <input type="hidden">
 <input type="text">
 <input type="search">
 <input type="tel">
 <input type="url">
 <input type="email">
 <input type="password">
 <input type="date">
 <input type="month">
 <input type="week">
 <input type="time">
 <input type="datetime-local">
 <input type="number">
 <input type="range">
 <input type="color">
 <input type="checkbox">
 <input type="radio">
 <input type="file">
 <input type="submit">
 <input type="image">
 <input type="reset">
 <input type="button">
 <select><optgroup><option></select>
 <select multiple></select>
 <optgroup></optgroup>
 <option></option>
 <button></button>
 <textarea></textarea>
 <table><tbody><tr><td></table>
 <marquee></marquee>
</div>

<div id="refs"></div>

<script>
 const props = ['letter-spacing',
                'word-spacing',
                'line-height',
                'text-transform',
                'text-indent',
                'text-shadow',
                'text-align',
                'display',
                'overflow',
                'overflow-clip-margin',
                'box-sizing'];
 runUAStyleTests(props);

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2627,
        "byte_start": 2607,
        "col": 2,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2672,
        "byte_start": 2651,
        "col": 2,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 2779,
        "byte_start": 2770,
        "col": 10,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 2864,
        "byte_start": 2855,
        "col": 2,
        "line": 100
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/resets.html"
}
```
