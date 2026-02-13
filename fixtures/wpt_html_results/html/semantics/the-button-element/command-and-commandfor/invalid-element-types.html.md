# html/semantics/the-button-element/command-and-commandfor/invalid-element-types.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/invalid-element-types.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long" />
<meta charset="utf-8" />
<meta name="author" href="mailto:masonf@chromium.org" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>
<script src="/html/resources/common.js"></script>

<meta name=variant content=?command=--custom-event&half=first>
<meta name=variant content=?command=--custom-event&half=second>
<meta name=variant content=?command=show-popover&half=first>
<meta name=variant content=?command=show-popover&half=second>
<meta name=variant content=?command=show-modal&half=first>
<meta name=variant content=?command=show-modal&half=second>

<div id="invokee"></div>

<script>
  // The command parameter is provided by variants, to
  // effectively split this (slow) test into multiple runs.
  const urlParams = new URLSearchParams(window.location.search);
  command = urlParams.get('command');
  half = urlParams.get('half');
  const firstHalf = half === 'first';
  assert_true(firstHalf || half === 'second');

  const allowed = ['button','menuitem'];
  const allTags = HTML5_ELEMENTS.filter(el => (!allowed.includes(el)));
  const midpoint = Math.floor(allTags.length / 2);
  const tags = firstHalf ? allTags.slice(0,midpoint) : allTags.slice(midpoint);
  let gotEvent = false;
  invokee.addEventListener('command',() => (gotEvent = true));
  for(const tag of tags) {
    promise_test(async () => {
      gotEvent = false;
      const element = document.createElement(tag);
      element.setAttribute('commandfor','invokee');
      element.setAttribute('command',command);
      element.style.display = 'block'; // For normally invisible elements
      document.body.appendChild(element);
      // Click two ways
      element.click();
      await clickOn(element);
      assert_false(gotEvent,'Command should not be fired');
    },`command/commandfor on <${tag}> with command=${command} should not function`);
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 136,
        "byte_start": 80,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 136,
        "byte_start": 80,
        "col": 1,
        "line": 4
      }
    },
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/invalid-element-types.html"
}
```
