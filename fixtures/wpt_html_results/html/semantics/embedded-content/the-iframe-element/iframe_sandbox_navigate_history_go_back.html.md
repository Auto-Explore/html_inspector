# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigate_history_go_back.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigate_history_go_back.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Check that sandboxed iframe can not navigate their ancestors</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  var t = async_test();
  onpopstate = t.unreached_func('no pop state');

  function doNavigation() {
    history.pushState( {state: "one past"}, 'page 2', '');
    frames[0].postMessage('back', '*');
    t.step_timeout(() => {
      t.done();
    }, 1000);
  }
</script>
<iframe id="child_frame" sandbox="allow-scripts" src="support/iframe-tried-to-be-navigated-by-history.html" onload="doNavigation();"></iframe>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_navigate_history_go_back.html"
}
```
