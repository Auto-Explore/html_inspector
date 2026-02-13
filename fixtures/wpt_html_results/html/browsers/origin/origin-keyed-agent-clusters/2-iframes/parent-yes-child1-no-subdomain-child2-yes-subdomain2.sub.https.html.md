# html/browsers/origin/origin-keyed-agent-clusters/2-iframes/parent-yes-child1-no-subdomain-child2-yes-subdomain2.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/2-iframes/parent-yes-child1-no-subdomain-child2-yes-subdomain2.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is origin-keyed, subdomain child 1 is site-keyed, different-subdomain child 2 is origin-keyed</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  testDifferentAgentClusters,
  testGetter
} from "../resources/helpers.mjs";

promise_setup(async () => {
  // Order of loading should not matter, but we make it sequential to ensure the
  // tests are deterministic.
  await insertIframe("{{hosts[][www]}}");
  await insertIframe("{{hosts[][www1]}}", "?1");
});


// Since everybody is different-origin, everyone's requests/non-requests get
// respected.
//
// So, the parent ends up in its origin-keyed agent cluster, child 1 ends up in
// the site-keyed agent cluster, and child 2 ends up in a different origin-keyed
// agent cluster.
testDifferentAgentClusters([self, 0], "Parent to child1");
testDifferentAgentClusters([self, 1], "Parent to child2");
testDifferentAgentClusters([0, 1], "child1 to child2");
testDifferentAgentClusters([1, 0], "child2 to child1");

testGetter(self, true, "parent");
testGetter(0, false, "child1");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/2-iframes/parent-yes-child1-no-subdomain-child2-yes-subdomain2.sub.https.html"
}
```
