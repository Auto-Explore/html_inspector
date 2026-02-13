# html/browsers/origin/origin-keyed-agent-clusters/popups/opener-yes-openee-yes-port.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/popups/opener-yes-openee-yes-port.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Opener is origin-keyed, openee is origin-keyed, openee is different-origin to the opener because of a port mismatch</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  openWindow,
  testOpenedWindowIsInADifferentAgentCluster,
  testGetter
} from "../resources/helpers.mjs";

let openee;
promise_setup(async () => {
  openee = await openWindow("{{hosts[][]}}:{{ports[https][1]}}", "?1");
});

// Both request origin-keying, so the opener ends up in one origin-keyed agent
// cluster (the default port's origin), and the openee ends up in a different
// origin-keyed agent cluster (the other port's origin).
testOpenedWindowIsInADifferentAgentCluster(() => openee);

testGetter(self, true, "opener");
testGetter(() => openee, true, "openee");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/popups/opener-yes-openee-yes-port.sub.https.html"
}
```
