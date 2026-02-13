# html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-no-1-subdomain-yes-2-subdomain2-no.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-no-1-subdomain-yes-2-subdomain2-no.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is site-keyed, navigate a frame from a subdomain origin-keyed to a second-subdomain site-keyed</title>
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
  frame1 = await insertIframe("{{hosts[][www]}}", "?1");
});

// Since they are different-origin, the child's origin-keying request is
// respected.

testDifferentAgentClusters([self, 0], "Before: parent to child");
testGetter(self, false, "before parent");
testGetter(0, true, "before child");

promise_test(async () => {
  await navigateIframe(frame1, "{{hosts[][www1]}}");
  await setBothDocumentDomains(frames[0]);
}, "Navigation");

// Make sure that the different-subdomain page (which doesn't request
// origin-keying) doesn't somehow get origin-keyed just because its predecessor
// was.

testSameAgentCluster([self, 0], "After: parent to child");
testGetter(self, false, "after parent");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-no-1-subdomain-yes-2-subdomain2-no.sub.https.html"
}
```
