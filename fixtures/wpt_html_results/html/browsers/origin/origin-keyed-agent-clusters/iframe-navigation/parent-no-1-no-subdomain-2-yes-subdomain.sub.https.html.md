# html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-no-1-no-subdomain-2-yes-subdomain.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-no-1-no-subdomain-2-yes-subdomain.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is site-keyed, navigate a frame from a subdomain site-keyed to the same subdomain origin-keyed</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  navigateIframe,
  setBothDocumentDomains,
  testSameAgentCluster,
  testGetter
} from "../resources/helpers.mjs";

let frame1;
promise_setup(async () => {
  frame1 = await insertIframe("{{hosts[][www]}}");
});

// Nobody requested origin-keying yet.

testSameAgentCluster([self, 0], "Before: parent to child");
testGetter(self, false, "before parent");
testGetter(0, false, "before child");

promise_test(async () => {
  await navigateIframe(frame1, "{{hosts[][www]}}", "?1");
  await setBothDocumentDomains(frames[0]);
}, "Navigation");

// Because this subdomain was previously site-keyed, the second load's
// origin-keying request is ignored; instead we continue with site-keying.

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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/iframe-navigation/parent-no-1-no-subdomain-2-yes-subdomain.sub.https.html"
}
```
