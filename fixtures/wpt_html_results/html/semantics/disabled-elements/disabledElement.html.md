# html/semantics/disabled-elements/disabledElement.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/disabled-elements/disabledElement.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Disabled elements</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#disabled-elements">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<button disabled>button</button>
<input disabled>
<select disabled>
 <optgroup label="options" disabled>
  <option value="option1" disabled>option1
  <option value="option2">option2
</select>
<textarea disabled>textarea</textarea>
<fieldset disabled>
 <input type=radio name=c value=0 checked>
 <input type=radio name=c value=1>
</fieldset>
<a href="http://www.w3.org/" disabled>w3</a>
<span tabindex=0 disabled>foobar</span>

<script>
  test(function(){
    assert_equals(document.activeElement, document.body);
  }, "The body element must be the active element if no element is focused");

  ["button", "input", "select", "optgroup", "option", "textarea", "input[type=radio]"].forEach(function(el) {
    test(function() {
      var element = document.querySelector(el);
      element.focus();
      assert_equals(document.activeElement, document.body, "activeElement after focus on a disabled <" + el + "> remains unchanged");
    }, "A disabled <" + el + "> should not be focusable");
  });

  ["a", "span"].forEach(function(el) {
    test(function() {
      var element = document.querySelector(el);
      element.focus();
      assert_equals(document.activeElement, element, "focus on a <" + el + "> with a disabled attribute should make it the activeElement");
    }, "A disabled <" + el + "> should be focusable");
  });
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
  "source_name": "html/semantics/disabled-elements/disabledElement.html"
}
```
