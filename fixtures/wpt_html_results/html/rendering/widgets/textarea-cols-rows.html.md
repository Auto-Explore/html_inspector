# html/rendering/widgets/textarea-cols-rows.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/textarea-cols-rows.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-textarea-element-2">
<title>Test `cols` `rows` attributes behaivor</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>

<textarea id="missing"></textarea>
<textarea id="invalid" cols="-1" rows="-1"></textarea>
<textarea id="computed" style="border:none; padding:0;" cols="19" rows="5"></textarea>
<textarea id="computedNone" style="display:none" cols="17" rows="7"></textarea>

<textarea id="cols0" cols="0"></textarea>
<textarea id="cols1" cols="1"></textarea>
<textarea id="cols10" cols="10"></textarea>
<textarea id="cols20" cols="20"></textarea>
<textarea id="cols21" cols="21"></textarea>

<textarea id="rows0" rows="0"></textarea>
<textarea id="rows1" rows="1"></textarea>
<textarea id="rows2" rows="2"></textarea>
<textarea id="rows3" rows="3"></textarea>

<script>
test(() => {
  assert_equals(missing.offsetWidth, cols20.offsetWidth);
  assert_equals(missing.offsetHeight, rows2.offsetHeight);
}, 'A misssing attribute is equivalent to cols=20 rows=2');

test(() => {
  assert_equals(invalid.offsetWidth, cols20.offsetWidth);
  assert_equals(invalid.offsetHeight, rows2.offsetHeight);
  assert_equals(cols0.offsetWidth, cols20.offsetWidth);
  assert_equals(rows0.offsetHeight, rows2.offsetHeight);
}, 'An invalid attribute value is equivalent to cols=20 rows=2');

test(() => {
  assert_less_than(cols1.offsetWidth, cols10.offsetWidth, '1 < 10');
  assert_less_than(cols10.offsetWidth, cols20.offsetWidth, '10 < 20');
  assert_less_than(cols20.offsetWidth, cols21.offsetWidth, '20 < 21');
}, 'The width depends on a cols attribute');

test(() => {
  assert_less_than(rows1.offsetHeight, rows2.offsetHeight, '1 < 2');
  assert_less_than(rows2.offsetHeight, rows3.offsetHeight, '2 < 3');
}, 'The height depends on a rows attribute');

test(() => {
  const computedWidth = getComputedStyle(computed).width;
  assert_equals(computed.offsetWidth,
      Math.round(computedWidth.substring(0, computedWidth.length - 2)));

  const computedHeight = getComputedStyle(computed).height;
  assert_equals(computed.offsetHeight,
      Math.round(computedHeight.substring(0, computedHeight.length - 2)));
}, 'Cols/rows attribute values affect layout-dependent computed style');

test(() => {
  const computedNoneStyle = getComputedStyle(computedNone);
  assert_equals(computedNoneStyle.width, 'auto');
  assert_equals(computedNoneStyle.height, 'auto');
}, 'Cols/rows attribute values are not presentational hints');
</script>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.textarea.rows.positive",
      "message": "Bad value “-1” for attribute “rows” on element “textarea”.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 299,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.textarea.cols.positive",
      "message": "Bad value “-1” for attribute “cols” on element “textarea”.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 299,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.textarea.cols.positive",
      "message": "Bad value “0” for attribute “cols” on element “textarea”.",
      "severity": "Warning",
      "span": {
        "byte_end": 552,
        "byte_start": 522,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.textarea.rows.positive",
      "message": "Bad value “0” for attribute “rows” on element “textarea”.",
      "severity": "Warning",
      "span": {
        "byte_end": 769,
        "byte_start": 739,
        "col": 1,
        "line": 19
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
  "source_name": "html/rendering/widgets/textarea-cols-rows.html"
}
```
