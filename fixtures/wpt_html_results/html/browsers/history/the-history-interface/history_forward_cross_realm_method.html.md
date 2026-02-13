# html/browsers/history/the-history-interface/history_forward_cross_realm_method.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/history_forward_cross_realm_method.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>history.forward() uses this's associated document's browsing context to determine if navigation is allowed</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/history.html#dom-history-forward">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe id="sandboxedIframe" srcdoc="hello" sandbox="allow-scripts allow-same-origin"></iframe>
<script>
const t = async_test();

t.step(() => {
  history.pushState({}, null, "?prev");
  history.pushState({}, null, "?current");
  history.back();
});

let isCrossRealmForward = false;
window.onpopstate = t.step_func(() => {
  if (isCrossRealmForward) {
    assert_equals(location.search, "?current");
    t.done();
  } else {
    sandboxedIframe.contentWindow.history.forward.call(history);
    isCrossRealmForward = true;
  }
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.sandbox.scripts_and_same_origin",
      "message": "Bad value “allow-scripts allow-same-origin” for attribute “sandbox” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 365,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/history/the-history-interface/history_forward_cross_realm_method.html"
}
```
