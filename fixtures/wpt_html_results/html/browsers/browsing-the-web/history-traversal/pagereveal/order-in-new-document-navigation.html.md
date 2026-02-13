# html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-new-document-navigation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-new-document-navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>pagereveal event fires and in correct order on new-document navigation</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#update-the-rendering">
<link rel="author" href="mailto:bokan@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
const event_log = [];

addEventListener('pagereveal', () => event_log.push('pagereveal'));
requestAnimationFrame(() => event_log.push('rAF'));

promise_test(async () => {
  await new Promise(resolve => requestAnimationFrame(resolve));
  assert_equals(event_log.toString(),'pagereveal,rAF');
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pagereveal/order-in-new-document-navigation.html"
}
```
