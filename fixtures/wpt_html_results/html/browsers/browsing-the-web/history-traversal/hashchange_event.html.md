# html/browsers/browsing-the-web/history-traversal/hashchange_event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/hashchange_event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Queue a task to fire hashchange event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
t = async_test();
window.onload = t.step_func(function () {
  if (location.href.toString().indexOf("#") > -1) {
    location.href = location.href.replace(/#.*$/,'');
    return;
  }
  var root = location.href;
  var oldURLs = [];
  var newURLs = [];

  var timer = null;

  location.hash = 'foo';
  window.onhashchange = t.step_func(function (e) {
    assert_true(e.isTrusted, "isTrusted");
    assert_equals(e.target, window, "target");
    assert_equals(e.type, "hashchange", "type");
    assert_true(e instanceof HashChangeEvent, "is HashChangeEvent");
    assert_false(e.bubbles, "bubbles");
    assert_false(e.cancelable, "cancelable");
    oldURLs.push(e.oldURL);
    newURLs.push(e.newURL);
    if (newURLs.length === 2) {
      check_result();
    } else if (timer === null) {
      timer = setTimeout(function() {check_result()}, 500);
    }
  })

  check_result = t.step_func(function() {
    clearTimeout(timer);
    try {
      assert_array_equals([root, root+"#foo"], oldURLs, "e.newURL");
      assert_array_equals([root+"#foo", root+"#bar"], newURLs, "e.newURL");
      t.done();
    } finally {
      location.hash = "";
    }
  });

  location.hash = 'bar';
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/hashchange_event.html"
}
```
