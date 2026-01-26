# Data Model: AI 对话 Tab

## Entity: Session

- **id**: integer, auto-increment
- **session_id**: UUID, unique
- **title**: string, optional
- **created_at**: timestamp
- **updated_at**: timestamp

**Relationships**:
- One Session has many SessionChatMessage records.

## Entity: SessionChatMessage

- **id**: integer, auto-increment
- **session_id**: UUID (references Session.session_id)
- **role**: enum (user, assistant)
- **content**: text
- **created_at**: timestamp
- **updated_at**: timestamp
- **saved_state**: enum (saved, unsaved) to mark persistence failures

**Validation rules**:
- content must be non-empty after trimming.

## Entity: Agent

- **id**: integer, auto-increment
- **name**: string
- **description**: text, optional
- **created_at**: timestamp
- **updated_at**: timestamp

**Relationships**:
- Agent can be associated to SessionChatMessage generation (logical association).
