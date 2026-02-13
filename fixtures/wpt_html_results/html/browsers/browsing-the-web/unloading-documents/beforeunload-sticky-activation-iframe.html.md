# html/browsers/browsing-the-web/unloading-documents/beforeunload-sticky-activation-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/beforeunload-sticky-activation-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Beforeunload must be gated behind sticky activation: nested browsing context</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<p>If you happen to be running this test as a human, then be sure not to interact with any part of the page; that would invalidate the results!

<script>
setup({ single_test: true });

const iframe = document.createElement('iframe');
iframe.src = 'support/beforeunload-sticky-start.html';

window.onmessage = e => {
  assert_equals(e.data, 'navigated successfully');

  const desiredURL = (new URL('support/beforeunload-sticky-destination.html', location.href)).href;
  assert_equals(iframe.contentWindow.location.href, desiredURL);

  done();
};

document.body.append(iframe);
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/beforeunload-sticky-activation-iframe.html"
}
```
