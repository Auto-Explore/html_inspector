# html/dom/elements/elements-in-the-dom/historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/elements-in-the-dom/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical HTMLElement features</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<body>
<script>
[
  // https://github.com/whatwg/html/commit/389ec2620d89e9480ef8847bf016abdfa92427bc
  "commandType",
  "commandLabel",
  "commandIcon",
  "commandHidden",
  "commandDisabled",
  "commandChecked",
  "commandTriggers",
  // https://github.com/whatwg/html/pull/2402
  "dropzone",
].forEach(function(member) {
  test(function() {
    assert_false(member in document.body);
    assert_false(member in document.createElement('div'));
  }, 'HTMLElement member must be nuked: ' + member);
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
  "source_name": "html/dom/elements/elements-in-the-dom/historical.html"
}
```
