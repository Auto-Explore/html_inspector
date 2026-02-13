# html/semantics/forms/the-form-element/form-elements-nameditem-02.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-elements-nameditem-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>form.elements: parsing</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-form-elements">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#parsing-main-intr">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<form id=form>
<table>
<tr>
<td><input type="radio" name="radio1" id="r1" value=1></td>
<td><input type="radio" name="radio2" id="r2" value=2></td>
<input type="radio"  name="radio0" id="r0" value=0>
</tr>
</table>
</form>
</div>
<script>
test(function() {
  var form = document.getElementById("form");
  assert_array_equals(form.elements,
                      [document.getElementById("r0"),
                       document.getElementById("r1"),
                       document.getElementById("r2")]);
}, "form.elements should work correctly in the face of table syntax errors")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.input_in_table",
      "message": "Start tag “input” seen in “table”.",
      "severity": "Error",
      "span": {
        "byte_end": 628,
        "byte_start": 577,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/forms/the-form-element/form-elements-nameditem-02.html"
}
```
