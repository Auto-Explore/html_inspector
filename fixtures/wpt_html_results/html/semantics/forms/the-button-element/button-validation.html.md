# html/semantics/forms/the-button-element/button-validation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-validation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>button element validation</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-button-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<button id=btn1>button</button>
<button id=btn2 type=submit>button</button>
<button id=btn3 type=reset>button</button>
<button id=btn4 type=button>button</button>
<button id=btn5 type=menu>button</button>
<button id=btn6 type=foobar>button</button>
<script>
  function willValid(element, expectedType, willValidate, desc) {
    test(function(){
      assert_equals(element.type, expectedType);
      assert_equals(element.willValidate, willValidate);
    }, desc);
  }

  willValid(document.getElementById('btn1'), "submit", true, "missing type attribute");
  willValid(document.getElementById('btn2'), "submit", true, "submit type attribute");
  willValid(document.getElementById('btn3'), "reset", false, "reset type attribute");
  willValid(document.getElementById('btn4'), "button", false, "button type attribute");
  willValid(document.getElementById('btn5'), "submit", true, "historical menu type attribute");
  willValid(document.getElementById('btn6'), "submit", true, "invalid type attribute");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-button-element/button-validation.html"
}
```
