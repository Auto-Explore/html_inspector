# html/semantics/selectors/pseudo-classes/valid-invalid-fieldset-disconnected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/valid-invalid-fieldset-disconnected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:valid, :invalid) on disconnected fieldset element</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/semantics-other.html#selector-valid">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<fieldset id=fieldset1>
  <input id=input1>
</fieldset>

<fieldset id=fieldset2>
  <select id=select1 required multiple>
    <option>foo
  </select>
</fieldset>

<script>
test(() => {
  const fieldset = document.querySelector("#fieldset1");
  const input = document.querySelector("#input1");

  assert_true(fieldset.matches(":valid"));
  assert_false(fieldset.matches(":invalid"));

  fieldset.remove();
  input.setCustomValidity("foo");

  assert_false(fieldset.matches(":valid"));
  assert_true(fieldset.matches(":invalid"));

  input.setCustomValidity("");

  assert_true(fieldset.matches(":valid"));
  assert_false(fieldset.matches(":invalid"));
}, "<input> element becomes invalid inside disconnected <fieldset>");

test(() => {
  const fieldset = document.querySelector("#fieldset2");
  const select = document.querySelector("#select1");

  assert_false(fieldset.matches(":valid"));
  assert_true(fieldset.matches(":invalid"));

  fieldset.remove();
  select.required = false;

  assert_true(fieldset.matches(":valid"));
  assert_false(fieldset.matches(":invalid"));

  select.required = true;
  select.firstElementChild.selected = true;

  assert_true(fieldset.matches(":valid"));
  assert_false(fieldset.matches(":invalid"));
}, "<select> element becomes valid inside disconnected <fieldset>");
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
  "source_name": "html/semantics/selectors/pseudo-classes/valid-invalid-fieldset-disconnected.html"
}
```
