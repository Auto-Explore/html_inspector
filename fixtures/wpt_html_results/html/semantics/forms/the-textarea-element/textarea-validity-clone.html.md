# html/semantics/forms/the-textarea-element/textarea-validity-clone.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-validity-clone.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTML Test: &lt;textarea&gt; validity state is correct after a clone</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-textarea-element">
<link rel="help" href="https://bugzil.la/1472169">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
  let form = document.createElement("form");
  let textarea = document.createElement("textarea");
  textarea.required = true;

  textarea.appendChild(document.createTextNode("A"));
  form.appendChild(textarea);

  assert_true(textarea.validity.valid);

  let formClone = form.cloneNode(true);
  assert_equals(
    formClone.querySelector('textarea').validity.valid,
    textarea.validity.valid,
    "Validity state should be preserved after a clone"
  );
}, "<textarea> validity state should be preserved after a clone");
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-validity-clone.html"
}
```
