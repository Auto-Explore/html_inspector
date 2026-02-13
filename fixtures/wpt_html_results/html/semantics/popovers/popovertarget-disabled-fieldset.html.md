# html/semantics/popovers/popovertarget-disabled-fieldset.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popovertarget-disabled-fieldset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/8221#discussion_r987582567">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=popover popover=auto>popover</div>

<fieldset>
  <button id=popoverbutton popovertarget=popover>open popover with popovertarget</button>
  <button id=commandbutton commandfor=popover command=show-popover>open popover with command</button>
</fieldset>

<script>
test(() => {
  const popover = document.getElementById('popover');
  const fieldset = document.querySelector('fieldset');
  ['popoverbutton', 'commandbutton'].forEach(id => {
    const button = document.getElementById(id);
    button.click();
    assert_true(popover.matches(':popover-open'),
      `${id}: popover should open when not disabled.`);
    popover.hidePopover();

    fieldset.disabled = true;
    button.click();
    assert_false(popover.matches(':popover-open'),
      `${id}: popover should not open when disabled.`);
    fieldset.disabled = false;
  });
}, 'Disabled fieldset ancestor should prevent popovertarget/command behavior.');
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
  "source_name": "html/semantics/popovers/popovertarget-disabled-fieldset.html"
}
```
