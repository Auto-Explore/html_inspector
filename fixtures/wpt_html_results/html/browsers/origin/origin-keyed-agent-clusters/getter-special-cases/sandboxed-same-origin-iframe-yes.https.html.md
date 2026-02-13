# html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/sandboxed-same-origin-iframe-yes.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/sandboxed-same-origin-iframe-yes.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>window.originAgentCluster for a sandboxed, but same-origin, iframe on an origin-keyed page</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import runTest from "./resources/sandboxed-same-origin-iframe-test.sub.mjs";
runTest({ expected: true });
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/sandboxed-same-origin-iframe-yes.https.html"
}
```
