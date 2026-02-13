# html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-yes-same.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-yes-same.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is site-keyed, child is origin-keyed, child is same-origin to the parent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  testSameAgentCluster,
  testGetter
} from "../resources/helpers.mjs";

promise_setup(async () => {
  await insertIframe("{{hosts[][]}}", "?1");
});

// Since they're same-origin, and the parent loaded in the site-keyed agent
// cluster, the child's request for origin-keying gets ignored, and both end up
// site-keyed.
testSameAgentCluster([self, 0]);

testGetter(self, false, "parent");
testGetter(0, false, "child");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-yes-same.sub.https.html"
}
```
