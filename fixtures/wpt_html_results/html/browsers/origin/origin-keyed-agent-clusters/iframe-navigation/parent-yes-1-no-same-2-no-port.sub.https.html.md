# html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-yes-1-no-same-2-no-port.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-yes-1-no-same-2-no-port.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is origin-keyed, navigate a frame from same-origin site-keyed to different-origin (different-port) origin-keyed</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  navigateIframe,
  setBothDocumentDomains,
  testSameAgentCluster,
  testDifferentAgentClusters,
  testGetter
} from "../resources/helpers.mjs";

let frame1;
promise_setup(async () => {
  frame1 = await insertIframe("{{hosts[][]}}");
});

// Since the parent is origin-keyed, the same-origin child's non-request is
// ignored, so it gets origin-keyed too.

testSameAgentCluster([self, 0], "Before: parent to child");
testGetter(self, true, "before parent");
testGetter(0, true, "before child");

promise_test(async () => {
  await navigateIframe(frame1, "{{hosts[][]}}:{{ports[https][1]}}");
  await setBothDocumentDomains(frames[0]);
}, "Navigation");

// Since the new page is different-origin, its non-request should be respected.

testDifferentAgentClusters([self, 0], "After: parent to child");
testGetter(self, true, "after parent");
testGetter(0, false, "after child");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-yes-1-no-same-2-no-port.sub.https.html"
}
```
