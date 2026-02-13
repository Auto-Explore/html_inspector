# html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-of-promise-result.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-of-promise-result.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>import() inside compiled strings inside a classic script</title>
<link rel="help" href="https://github.com/whatwg/html/pull/3163">
<link rel="help" href="https://github.com/tc39/ecma262/issues/871#issuecomment-292493142">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<base href="scripts/foo/">
<script>
"use strict";

// This test is based on the current specification, but all browser
// implementations aren't conformant. See
// https://bugs.chromium.org/p/chromium/issues/detail?id=1245063
// https://github.com/heycam/webidl/pull/902

// Tweak the base URL of the document here to distinguish:
// - document URL
// - document base URL = ../
// - This inline script's base URL = ./scripts/foo/
document.querySelector("base").remove();
const base = document.createElement("base");
base.setAttribute("href", "../");
document.body.appendChild(base);

self.ran = false;

// The active script for the dynamic import is this inline script
// (see https://html.spec.whatwg.org/C/#hostmakejobcallback).
promise_test(t => {
  t.add_cleanup(() => {
    self.ran = false;
  })

  return Promise.resolve(`import("../../../imports-a.js?1").then(() => { self.ran = true; })`)
    .then(eval)
    .then(() => {
      assert_true(self.ran);
    });
}, "Evaled the script via eval, successful import");

promise_test(t => {
  t.add_cleanup(() => {
    self.ran = false;
  })

  return Promise.resolve(`import("bad-specifier?1").catch(() => { self.ran = true; })`)
    .then(eval)
    .then(() => {
      assert_true(self.ran);
    });
}, "Evaled the script via eval, failed import");

promise_test(t => {
  t.add_cleanup(() => {
    self.ran = false;
  })

  return Promise.resolve(`return import("../../../imports-a.js?2").then(() => { self.ran = true; })`)
    .then(Function)
    .then(Function.prototype.call.bind(Function.prototype.call))
    .then(() => {
      assert_true(self.ran);
    });
}, "Evaled the script via Function, successful import");

promise_test(t => {
  t.add_cleanup(() => {
    self.ran = false;
  })

  return Promise.resolve(`return import("bad-specifier?2").catch(() => { self.ran = true; })`)
    .then(Function)
    .then(Function.prototype.call.bind(Function.prototype.call))
    .then(() => {
      assert_true(self.ran);
    });
}, "Evaled the script via Function, failed import");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.not_in_body",
      "message": "Element “base” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 481,
        "byte_start": 455,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 481,
        "byte_start": 455,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-of-promise-result.html"
}
```
