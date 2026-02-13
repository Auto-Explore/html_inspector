# html/semantics/selectors/pseudo-classes/indeterminate-radio.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/indeterminate-radio.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>:indeterminate and input type=radio</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style type="text/css">
#test {
  color: green;
}
input:indeterminate + #test {
  color: red;
}
</style>
<input type="radio" name="radios">
<div id="test"></div>
<input type="radio" name="radios" checked>
<script type="text/javascript">
test(function() {
  document.getElementsByTagName("input")[0].indeterminate = true;
  var target = document.getElementById("test");
  var val = getComputedStyle(target, null).getPropertyValue("color");
  assert_equals(val, "rgb(0, 128, 0)",
                "The indeterminate IDL attribute should not cause the " +
                ":indeterminate pseudo-class to match on input type=radio");
})
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 219,
        "byte_start": 196,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 432,
        "byte_start": 401,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/selectors/pseudo-classes/indeterminate-radio.html"
}
```
