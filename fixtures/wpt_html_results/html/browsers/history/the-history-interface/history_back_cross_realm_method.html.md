# html/browsers/history/the-history-interface/history_back_cross_realm_method.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/history_back_cross_realm_method.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>history.back() uses this's associated document's browsing context to determine if navigation is allowed</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/history.html#dom-history-back">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe id="sandboxedIframe" srcdoc="hello" sandbox="allow-scripts allow-same-origin"></iframe>
<script>
const t = async_test();

t.step(() => {
  history.pushState({}, null, "?prev");
  history.pushState({}, null, "?current");

  sandboxedIframe.contentWindow.history.back.call(history);
});

window.onpopstate = t.step_func_done(() => {
  assert_equals(location.search, "?prev");
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
        "byte_end": 445,
        "byte_start": 359,
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
  "source_name": "html/browsers/history/the-history-interface/history_back_cross_realm_method.html"
}
```
