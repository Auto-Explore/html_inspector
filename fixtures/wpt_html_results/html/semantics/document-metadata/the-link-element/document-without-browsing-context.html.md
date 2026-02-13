# html/semantics/document-metadata/the-link-element/document-without-browsing-context.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/document-without-browsing-context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Documents without browsing contexts should not load stylesheets</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<body>
<script>
  function count(id, t) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', 'stylesheet.py?count=1&id=' + id);
    xhr.onload = t.step_func_done(function() {
      assert_equals(xhr.responseText, "1");
    });
    xhr.onerror = t.unreached_func();
    xhr.send();
  }

  async_test(function(t) {
    var id = token();
    var doc = (new DOMParser()).parseFromString('<link rel="stylesheet" href="stylesheet.py?id=' + id + '"></link>', 'text/html');
    var link = doc.querySelector('link');
    document.head.appendChild(link);
    t.step_timeout(function() { count(id, t) }, 500);
  }, 'Create a document, adopt the node');

  async_test(function(t) {
    var id = token();
    var d = document.createElement('div');
    document.body.appendChild(d);
    d.innerHTML = '<link rel="stylesheet" href="stylesheet.py?id=' + id + '"></link>';
    t.step_timeout(function() { count(id, t) }, 500);
  }, 'Create a stylesheet in innerHTML document');
</script>
</body>
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
  "source_name": "html/semantics/document-metadata/the-link-element/document-without-browsing-context.html"
}
```
