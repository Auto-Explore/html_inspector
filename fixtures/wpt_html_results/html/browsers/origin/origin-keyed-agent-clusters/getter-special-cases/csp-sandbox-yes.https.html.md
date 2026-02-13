# html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/csp-sandbox-yes.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/csp-sandbox-yes.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>window.originAgentCluster for a top-level frame sandboxed by CSP with an Origin-Agent-Cluster header</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import { testGetter } from "../resources/helpers.mjs";

// We're definitely origin-keyed: both the CSP sandboxing and the
// Origin-Agent-Cluster header should ensure this.
testGetter(self, true);
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/csp-sandbox-yes.https.html"
}
```
