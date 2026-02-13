# html/browsers/origin/relaxing-the-same-origin-restriction/document_domain_access_details.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/relaxing-the-same-origin-restriction/document_domain_access_details.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/document_domain_frame.sub.js"></script>
<body>
<script>
promise_test(async (t) => {
  let frame1 = await createFrame(t, "control1-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "control1-2", "{{domains[www1]}}");
  let result = await postMessageToFrame(frame1, { 'poke-at-sibling': "control1-2" });
  assert_equals(result.data, "omg!");
}, "Access allowed if same-origin with no 'document.domain' modification. (Sanity check)");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "control2-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "control2-2", "{{domains[www2]}}");
  let result = await postMessageToFrame(frame1, { 'poke-at-sibling': "control2-2" });
  assert_equals(result.data, "SecurityError");
}, "Access not allowed if different-origin with no 'document.domain' modification. (Sanity check)");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "one-set-one-not-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "one-set-one-not-2", "{{domains[www1]}}");
  await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });

  let result = await postMessageToFrame(frame1, { 'poke-at-sibling': "one-set-one-not-2" });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "one-set-one-not-1" });
  assert_equals(result.data, "SecurityError");
}, "Access disallowed if same-origin but only one sets document.domain.");

promise_test(async (t) => {
  var frame1 = await createFrame(t, "both-set-to-existing-1", "{{domains[www1]}}");
  var frame2 = await createFrame(t, "both-set-to-existing-2", "{{domains[www1]}}");
  let result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame2, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, { 'poke-at-sibling': "both-set-to-existing-2" });
  assert_equals(result.data, "omg!");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "both-set-to-existing-1" });
  assert_equals(result.data, "omg!");
}, "Access allowed if same-origin and both set document.domain to existing value.");

promise_test(async (t) => {
  var frame1 = await createFrame(t, "both-set-to-parent-1", "{{domains[www1]}}");
  var frame2 = await createFrame(t, "both-set-to-parent-2", "{{domains[www2]}}");
  let result = await postMessageToFrame(frame1, { domain: "{{domains[]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame2, { domain: "{{domains[]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, { 'poke-at-sibling': "both-set-to-parent-2" });
  assert_equals(result.data, "omg!");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "both-set-to-parent-1" });
  assert_equals(result.data, "omg!");
}, "Access allowed if different-origin but both set document.domain to parent domain.");

promise_test(async (t) => {
  var frame1 = await createFrame(t, "allow-then-revoke-1", "{{domains[www1]}}");
  var frame2 = await createFrame(t, "allow-then-revoke-2", "{{domains[www1]}}");
  let result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame2, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, { 'poke-at-sibling': "allow-then-revoke-2" });
  assert_equals(result.data, "omg!");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "allow-then-revoke-1" });
  assert_equals(result.data, "omg!");

  result = await postMessageToFrame(frame1, { domain: "{{domains[]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, { 'poke-at-sibling': "allow-then-revoke-2" });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "allow-then-revoke-1" });
  assert_equals(result.data, "SecurityError");
}, "Access disallowed again if same-origin, both set document-domain to existing value, then one sets to parent.");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "revoke-Window-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "revoke-Window-2", "{{domains[www1]}}");

  let result = await postMessageToFrame(frame1, { cache: ["parent", "revoke-Window-2"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 1");

  result = await postMessageToFrame(frame2, { cache: ["parent", "revoke-Window-1"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 1");

  result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "SecurityError");
}, "Access is revoked to Window object when we stop being same effective script origin due to document.domain.");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "revoke-Location-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "revoke-Location-2", "{{domains[www1]}}");

  let result = await postMessageToFrame(frame1, { cache: ["parent", "revoke-Location-2", "location"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 3");

  result = await postMessageToFrame(frame2, { cache: ["parent", "revoke-Location-1", "location"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 3");

  result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "SecurityError");
}, "Access is revoked to Location object when we stop being same effective script origin due to document.domain.");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "no-revoke-Document-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "no-revoke-Document-2", "{{domains[www1]}}");

  let result = await postMessageToFrame(frame1, { cache: ["parent", "no-revoke-Document-2", "document"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 4");

  result = await postMessageToFrame(frame2, { cache: ["parent", "no-revoke-Document-1", "document"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "Reachable 4");

  result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 4");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "Reachable 4");
}, "Access is not revoked to Document object when we stop being same effective script origin due to document.domain.");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "no-revoke-object-1", "{{domains[www1]}}");
  let frame2 = await createFrame(t, "no-revoke-object-2", "{{domains[www1]}}");

  let result = await postMessageToFrame(frame1, { cache: ["parent", "no-revoke-object-2", "bar"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 2");

  result = await postMessageToFrame(frame2, { cache: ["parent", "no-revoke-object-1", "bar"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 2");

  result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 2");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "Reachable 2");
}, "Access is not revoked to random object when we stop being same effective script origin due to document.domain.");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "join-and-diverge-1", "{{domains[www2.www1]}}");
  let frame2 = await createFrame(t, "join-and-diverge-2", "{{domains[www1.www1]}}");

  // Make sure we can't touch each other.
  let result = await postMessageToFrame(frame1, { 'poke-at-sibling': "join-and-diverge-2" });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "join-and-diverge-1" });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame1, { cache: ["parent", "join-and-diverge-2", "bar"] });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, { cache: ["parent", "join-and-diverge-1", "document"] });
  assert_equals(result.data, "SecurityError");

  // Let's join up now.
  result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame2, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  // Now we should be able to touch each other.
  result = await postMessageToFrame(frame1, { 'poke-at-sibling': "join-and-diverge-2" });
  assert_equals(result.data, "omg!");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "join-and-diverge-1" });
  assert_equals(result.data, "omg!");

  // Cache a random object and a document.
  result = await postMessageToFrame(frame1, { cache: ["parent", "join-and-diverge-2", "bar"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 2");

  result = await postMessageToFrame(frame2, { cache: ["parent", "join-and-diverge-1", "document"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "Reachable 4");

  // OK, now let's diverge
  result = await postMessageToFrame(frame1, { domain: "{{domains[]}}" });
  assert_equals(result.data, "Done");

  // We should still be able to touch our cached things.
  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 2");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "Reachable 4");
}, "Access evolves correctly for non-cross-origin objects when we join up via document.domain and then diverge again.");

promise_test(async (t) => {
  let frame1 = await createFrame(t, "join-and-diverge-cross-origin-1", "{{domains[www2.www1]}}");
  let frame2 = await createFrame(t, "join-and-diverge-cross-origin-2", "{{domains[www1.www1]}}");

  // Make sure we can't touch each other.
  let result = await postMessageToFrame(frame1, { 'poke-at-sibling': "join-and-diverge-cross-origin-2" });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "join-and-diverge-cross-origin-1" });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame1, { cache: ["parent", "join-and-diverge-2", "bar"] });
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, { cache: ["parent", "join-and-diverge-1", "document"] });
  assert_equals(result.data, "SecurityError");

  // Let's join up now.
  result = await postMessageToFrame(frame1, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  result = await postMessageToFrame(frame2, { domain: "{{domains[www1]}}" });
  assert_equals(result.data, "Done");

  // Now we should be able to touch each other.
  result = await postMessageToFrame(frame1, { 'poke-at-sibling': "join-and-diverge-cross-origin-2" });
  assert_equals(result.data, "omg!");

  result = await postMessageToFrame(frame2, { 'poke-at-sibling': "join-and-diverge-cross-origin-1" });
  assert_equals(result.data, "omg!");

  // Cache a window and a location
  result = await postMessageToFrame(frame1, { cache: ["parent", "join-and-diverge-cross-origin-2"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "Reachable 1");

  result = await postMessageToFrame(frame2, { cache: ["parent", "join-and-diverge-cross-origin-1", "location"] });
  assert_equals(result.data, "cached");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "Reachable 3");

  // OK, now let's diverge
  result = await postMessageToFrame(frame1, { domain: "{{domains[]}}" });
  assert_equals(result.data, "Done");

  // Now our cross-origin objects should start denying access.
  result = await postMessageToFrame(frame1, 'touch-cached');
  assert_equals(result.data, "SecurityError");

  result = await postMessageToFrame(frame2, 'touch-cached');
  assert_equals(result.data, "SecurityError");
}, "Access evolves correctly for cross-origin objects when we join up via document.domain and then diverge again.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/origin/relaxing-the-same-origin-restriction/document_domain_access_details.sub.html"
}
```
