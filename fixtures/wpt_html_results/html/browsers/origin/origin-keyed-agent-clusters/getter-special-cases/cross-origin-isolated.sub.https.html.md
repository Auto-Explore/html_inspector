# html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/cross-origin-isolated.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/cross-origin-isolated.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>window.originAgentCluster must be implied by cross-origin isolation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="//{{domains[www1]}}:{{location[port]}}/html/browsers/origin/origin-keyed-agent-clusters/resources/coep-frame.html"></iframe>

<div id="log"></div>

<script type="module">
import { testGetter } from "../resources/helpers.mjs";

setup({ explicit_done: true });

window.onload = () => {
  // Cross-origin isolated pages are always origin-keyed.
  testGetter(self, true, "self");

  // Child frames of cross-origin isolated pages must also be cross-origin
  // isolated, and thus also origin-keyed. Make sure the implementation doesn't
  // treat them specially in some weird way, for the purposes of this
  // implication.
  testGetter(0, true, "child");

  done();
};
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/cross-origin-isolated.sub.https.html"
}
```
