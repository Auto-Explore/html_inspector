# html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-yeswithparams-subdomain.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-yeswithparams-subdomain.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is site-keyed, child is origin-keyed using parameters on its structured header, child is a subdomain of the parent</title>
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
  await insertIframe("{{hosts[][www]}}", "?1;param1;param2=value2");
});

// Since they're different-origin, the child's request is respected, so the
// parent ends up in the site-keyed agent cluster and the child in the
// origin-keyed one.
testDifferentAgentClusters([self, 0]);

testGetter(self, false, "parent");
testGetter(0, true, "child");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-yeswithparams-subdomain.sub.https.html"
}
```
