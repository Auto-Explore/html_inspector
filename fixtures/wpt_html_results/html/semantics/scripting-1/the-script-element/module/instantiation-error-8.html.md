# html/semantics/scripting-1/the-script-element/module/instantiation-error-8.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/instantiation-error-8.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Handling of instantiation errors, 8</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<!-- The below module tree should fail to instantiate, since it references undefined identifier. -->
<script type="module" src="instantiation-error-1.js"></script>
<script>
setup({allow_uncaught_exception: true});

promise_test(t => {
  return new Promise(resolve => {
    window.addEventListener("error", e => {
      assert_equals(e.error.constructor, SyntaxError);
      resolve();
    }, { once: true });
  }).then(() => new Promise(resolve => {
    window.addEventListener("error", e => {
      assert_equals(e.error.constructor, SyntaxError);
      resolve();
    }, { once: true });
    // Load another module tree w/ previously instantiate-failed tree as its sub-tree.
    document.head.appendChild(Object.assign(
        document.createElement('script'),
        { type: 'module', innerText: 'import "./instantiation-error-1.js"'}));
  }));
}, "Instantiate attempt on a tree w/ previously instantiate-failed tree as a sub-tree shouldn't crash.");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/instantiation-error-8.html"
}
```
