# html/semantics/the-button-element/command-and-commandfor/button-type-reflection.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/button-type-reflection.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:lwarlow@igalia.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe name=foo></iframe>
<form id="form" target=foo action="about:blank">
  <button id=reset-in-form type=reset commandfor=mypopover command=toggle-popover>reset</button>
  <button id=submit-in-form type=submit commandfor=mypopover command=toggle-popover>submit</button>
  <button id=button-in-form type=button commandfor=mypopover command=toggle-popover>type=button</button>
  <button id=invalid-in-form type=invalid commandfor=mypopover command=toggle-popover>invalid</button>
  <button id=invalid-in-form-command-only type=invalid command=toggle-popover>invalid with command only</button>
  <button id=invalid-in-form-commandfor-only type=invalid commandfor=mypopover >invalid with commandfor only</button>
  <button id=missing-in-form commandfor=mypopover command=toggle-popover>missing</button>
  <button id=missing-in-form-command-only command=toggle-popover>missing with command only</button>
  <button id=missing-in-form-commandfor-only commandfor=mypopover >missing with commandfor only</button>
</form>

<button id=reset-attr-form type=reset commandfor=mypopover command=toggle-popover form=form>reset</button>
<button id=submit-attr-form type=submit commandfor=mypopover command=toggle-popover form=form>submit</button>
<button id=button-attr-form type=button commandfor=mypopover command=toggle-popover form=form>type=button</button>
<button id=invalid-attr-form type=invalid commandfor=mypopover command=toggle-popover form=form>invalid</button>
<button id=invalid-attr-form-command-only type=invalid command=toggle-popover form=form>invalid with command only</button>
<button id=invalid-attr-form-commandfor-only type=invalid commandfor=mypopover form=form>invalid with commandfor only</button>
<button id=missing-attr-form commandfor=mypopover command=toggle-popover form=form>missing</button>
<button id=missing-attr-form-command-only command=toggle-popover form=form>missing with command only</button>
<button id=missing-attr-form-commandfor-only commandfor=mypopover form=form>missing with commandfor only</button>

<button id=reset-outside-form type=reset commandfor=mypopover command=toggle-popover>reset</button>
<button id=submit-outside-form type=submit commandfor=mypopover command=toggle-popover>submit</button>
<button id=button-outside-form type=button commandfor=mypopover command=toggle-popover>type=button</button>
<button id=invalid-outside-form type=invalid commandfor=mypopover command=toggle-popover>invalid</button>
<button id=invalid-outside-form-command-only type=invalid command=toggle-popover>invalid with command only</button>
<button id=invalid-outside-form-commandfor-only type=invalid commandfor=mypopover>invalid with commandfor only</button>
<button id=missing-outside-form commandfor=mypopover command=toggle-popover>missing</button>
<button id=missing-outside-form-command-only command=toggle-popover>missing with command only</button>
<button id=missing-outside-form-commandfor-only commandfor=mypopover>missing with commandfor only</button>

<script>
const data = {
  'reset-in-form': 'reset',
  'submit-in-form': 'submit',
  'button-in-form': 'button',
  'invalid-in-form': 'button',
  'invalid-in-form-command-only': 'button',
  'invalid-in-form-commandfor-only': 'button',
  'missing-in-form': 'button',
  'missing-in-form-command-only': 'button',
  'missing-in-form-commandfor-only': 'button',
  'reset-attr-form': 'reset',
  'submit-attr-form': 'submit',
  'button-attr-form': 'button',
  'invalid-attr-form': 'button',
  'invalid-attr-form-command-only': 'button',
  'invalid-attr-form-commandfor-only': 'button',
  'missing-attr-form': 'button',
  'missing-attr-form-command-only': 'button',
  'missing-attr-form-commandfor-only': 'button',
  'reset-outside-form': 'reset',
  'submit-outside-form': 'submit',
  'button-outside-form': 'button',
  'invalid-outside-form': 'button',
  'invalid-outside-form-command-only': 'button',
  'invalid-outside-form-commandfor-only': 'button',
  'missing-outside-form': 'button',
  'missing-outside-form-command-only': 'button',
  'missing-outside-form-commandfor-only': 'button',
};

Object.entries(data).map(([id, expectedType]) => {
  test(() => {
    const button = document.getElementById(id);
    assert_equals(button.type, expectedType, `type of ${id} should be ${expectedType}`);
  }, `Button with id ${id} should reflect type correctly`);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.commandfor.idref.missing",
      "message": "The value of the “commandfor” attribute of the “button” element must be the ID of an element in the same tree as the “button” with the “commandfor” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 332,
        "byte_start": 252,
        "col": 3,
        "line": 8
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/button-type-reflection.html"
}
```
