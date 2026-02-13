# html/semantics/forms/historical-search-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/historical-search-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>search event should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<input id=input type=search incremental>
<script>
promise_test(async t => {
  const input = document.getElementById('input');
  const eventWatcher = new EventWatcher(t, input, ['search', 'keypress']);
  await Promise.all([
    test_driver.send_keys(input, 'x'),
    eventWatcher.wait_for(['keypress'])
  ]);
  // During this timeout, the search event will fire, if it's supported,
  // which fails the test since the event watcher isn't expecting it.
  await new Promise(resolve => t.step_timeout(resolve, 1000));
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
  "source_name": "html/semantics/forms/historical-search-event.html"
}
```
