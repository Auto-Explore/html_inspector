# html/semantics/embedded-content/the-embed-element/embed-network-error.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-network-error.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Network errors with embed elements</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

async_test(t => {
  const embed = document.createElement("embed");
  embed.src = "//{{hosts[][nonexistent]}}/";
  embed.onload = () => t.done();
  embed.onerror = t.unreached_func("error event must not fire");
  document.body.append(embed);
}, "new embed: nonexistent host");

async_test(t => {
  const embed = document.createElement("embed");
  embed.src = "../resources/not-embeddable.html";
  embed.onload = () => t.done();
  embed.onerror = t.unreached_func("error event must not fire");
  document.body.append(embed);
}, "new embed: X-Frame-Options prevents embedding");

async_test(t => {
  const embed = document.createElement("embed");
  embed.src = "/common/blank.html";
  embed.name = "existingEmbed1";
  embed.onload = t.step_func(() => {
    embed.onload = () => t.done();
    embed.onerror = t.unreached_func("error event must not fire 2");

    frames.existingEmbed1.location.href = "//{{hosts[][nonexistent]}}/";
  });
  embed.onerror = t.unreached_func("error event must not fire 1");
  document.body.append(embed);
}, "navigating an existing embed: nonexistent host");

async_test(t => {
  const embed = document.createElement("embed");
  embed.src = "/common/blank.html";
  embed.name = "existingEmbed2";
  embed.onload = t.step_func(() => {
    embed.onload = () => t.done();
    embed.onerror = t.unreached_func("error event must not fire 2");

    frames.existingEmbed2.location.href = "../resources/not-embeddable.html";
  });
  embed.onerror = t.unreached_func("error event must not fire 1");
  document.body.append(embed);
}, "navigating an existing embed: X-Frame-Options prevents embedding");
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-network-error.sub.html"
}
```
