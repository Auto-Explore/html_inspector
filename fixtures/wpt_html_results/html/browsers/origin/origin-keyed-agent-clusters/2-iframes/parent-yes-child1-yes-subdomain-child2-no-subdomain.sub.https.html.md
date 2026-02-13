# html/browsers/origin/origin-keyed-agent-clusters/2-iframes/parent-yes-child1-yes-subdomain-child2-no-subdomain.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/2-iframes/parent-yes-child1-yes-subdomain-child2-no-subdomain.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is origin-keyed, subdomain child 1 is origin-keyed, same-subdomain child 2 is site-keyed</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  testSameAgentCluster,
  testDifferentAgentClusters,
  testGetter
} from "../resources/helpers.mjs";

promise_setup(async () => {
  // Must be sequential, not parallel: the origin-keyed frame must load first.
  await insertIframe("{{hosts[][www]}}", "?1");
  await insertIframe("{{hosts[][www]}}");
});


// Since they're different-origin, the parent's request is respected, as is
// child 1's request. child 2's non-request is ignored, since  child 1 is in the
// same browsing context group.
//
// So, the parent ends up in the origin-keyed agent cluster, and both children
// ends up in a different origin-keyed agent cluster.
testDifferentAgentClusters([self, 0], "Parent to child1");
testDifferentAgentClusters([self, 1], "Parent to child2");
testSameAgentCluster([0, 1], "child1 to child2");
testSameAgentCluster([1, 0], "child2 to child1");

testGetter(self, true, "parent");
testGetter(0, true, "child1");
testGetter(1, true, "child2");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/2-iframes/parent-yes-child1-yes-subdomain-child2-no-subdomain.sub.https.html"
}
```
