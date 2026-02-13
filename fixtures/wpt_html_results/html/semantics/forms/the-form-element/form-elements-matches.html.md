# html/semantics/forms/the-form-element/form-elements-matches.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-elements-matches.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>form.elements: matches</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-form-elements">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<form id="form">
<input type="image">
</form>
</div>
<script>
test(function() {
  assert_equals(document.getElementById("form").elements.length, 0);
}, "input type=image should not be present in the form.elements collection")
test(function() {
  var form = document.getElementById("form");
  var i = document.createElement("input");
  i.name = "2";
  form.appendChild(i);
  var j = document.createElement("input");
  j.name = "03";
  form.appendChild(j);
  assert_equals(form.elements[-1], undefined, '[-1]');
  assert_equals(form.elements["-1"], undefined, '["-1"]');
  assert_equals(form.elements[0], i, '[0]');
  assert_equals(form.elements["0"], i, '["0"]');
  assert_equals(form.elements[1], j, '[1]');
  assert_equals(form.elements["1"], j, '["1"]');
  assert_equals(form.elements[2], undefined, '[2]');
  assert_equals(form.elements["2"], undefined, '["2"]');
  assert_equals(form.elements[03], undefined, '[03]');
  assert_equals(form.elements["03"], j, '["03"]');
  assert_equals(form.elements.item(-1), null, 'item(-1)');
  assert_equals(form.elements.item(0), i, 'item(0)');
  assert_equals(form.elements.item(1), j, 'item(1)');
  assert_equals(form.elements.item(2), null, 'item(2)');
  assert_equals(form.elements.namedItem("2"), i, 'namedItem("2")');
  assert_equals(form.elements.namedItem("03"), j, 'namedItem("03")');
  assert_equals(form.elements.namedItem("3"), null, 'namedItem("3")');
  assert_array_equals(form.elements, [i, j]);
  form.removeChild(i);
  form.removeChild(j);
}, "form.elements should include elements whose name starts with a number");
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
        "byte_end": 383,
        "byte_start": 363,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/forms/the-form-element/form-elements-matches.html"
}
```
