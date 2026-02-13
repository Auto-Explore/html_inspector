# html/rendering/non-replaced-elements/form-controls/select-sizing-001.html

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/select-sizing-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Test for sizing of select elements, with wide vs. empty option selected</title>
  <link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#list-box">
  <link rel="match" href="select-sizing-001-ref.html">
  <style>
    select {
      color: transparent;
      margin: 1px;
    }
    div.customBorder > select {
      /* This class is to let us test select elements *without* native theming
         (for browsers that have both native and non-native controls): */
      border: 3px solid black;
    }
  </style>
</head>
<body>
  <div>
    <!-- Wide thing is 2nd, and not selected: -->
    <select>
      <option></option>
      <option>some wide option</option>
    </select>
    <br>
    <!-- Wide thing is 2nd, and selected: -->
    <select>
      <option></option>
      <option selected>some wide option</option>
    </select>
    <br>
    <!-- Wide thing is 1st, and selected (implicitly): -->
    <select>
      <option>some wide option</option>
      <option></option>
    </select>
    <br>
    <!-- Wide thing is 1st, and not selected: -->
    <select>
      <option>some wide option</option>
      <option selected></option>
    </select>
  </div>

  <!-- This is the same as above, but now with a custom border on the
       select elements: -->
  <div class="customBorder">
    <!-- Wide thing is 2nd, and not selected: -->
    <select>
      <option></option>
      <option>some wide option</option>
    </select>
    <br>
    <!-- Wide thing is 2nd, and selected: -->
    <select>
      <option></option>
      <option selected>some wide option</option>
    </select>
    <br>
    <!-- Wide thing is 1st, and selected (implicitly): -->
    <select>
      <option>some wide option</option>
      <option></option>
    </select>
    <br>
    <!-- Wide thing is 1st, and not selected: -->
    <select>
      <option>some wide option</option>
      <option selected></option>
    </select>
  </div>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 776,
        "col": 15,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 931,
        "byte_start": 922,
        "col": 15,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 1139,
        "byte_start": 1130,
        "col": 15,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 1298,
        "byte_start": 1289,
        "col": 24,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 1536,
        "byte_start": 1527,
        "col": 15,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 1682,
        "byte_start": 1673,
        "col": 15,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 1890,
        "byte_start": 1881,
        "col": 15,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 2049,
        "byte_start": 2040,
        "col": 24,
        "line": 72
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/select-sizing-001.html"
}
```
