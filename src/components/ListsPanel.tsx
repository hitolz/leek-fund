import React, { useState } from "react";
import { FundList } from "../types";
import { useTauriCommands } from "../hooks/useTauriCommands";

interface ListsPanelProps {
  lists: FundList[];
  selectedListId: number | null;
  onSelectList: (id: number | null) => void;
  onListsChange: () => void;
  showToast?: (message: string, type: "success" | "error") => void;
}

export const ListsPanel: React.FC<ListsPanelProps> = ({
  lists,
  selectedListId,
  onSelectList,
  onListsChange,
  showToast,
}) => {
  const [newListName, setNewListName] = useState("");
  const [creating, setCreating] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editingName, setEditingName] = useState("");
  const { createList, renameList, deleteList } = useTauriCommands();

  const handleCreateList = async () => {
    if (!newListName.trim()) {
      if (showToast) {
        showToast("列表名称不能为空", "error");
      }
      return;
    }

    setCreating(true);
    try {
      await createList(newListName.trim());
      if (showToast) {
        showToast("列表创建成功", "success");
      }
      setNewListName("");
      onListsChange();
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    } finally {
      setCreating(false);
    }
  };

  const handleRename = async (id: string) => {
    if (!editingName.trim()) {
      if (showToast) {
        showToast("列表名称不能为空", "error");
      }
      return;
    }

    try {
      await renameList(id, editingName.trim());
      if (showToast) {
        showToast("重命名成功", "success");
      }
      setEditingId(null);
      onListsChange();
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    }
  };

  const handleDelete = async (id: string, name: string, fundCount: number) => {
    const confirmMessage =
      fundCount > 0
        ? `确定删除列表"${name}"吗？将移除${fundCount}只基金。`
        : `确定删除列表"${name}"吗？`;

    if (!confirm(confirmMessage)) {
      return;
    }

    try {
      await deleteList(id);
      if (showToast) {
        showToast("列表已删除", "success");
      }
      if (selectedListId === id) {
        onSelectList(null);
      }
      onListsChange();
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    }
  };

  return (
    <div className="lists-panel">
      <div className="panel-header">
        <h3>我的列表</h3>
      </div>

      <div className="create-list-section">
        <input
          type="text"
          value={newListName}
          onChange={(e) => setNewListName(e.target.value)}
          placeholder="新建列表"
          maxLength={64}
          className="list-name-input"
          disabled={creating}
          onKeyPress={(e) => {
            if (e.key === "Enter") {
              handleCreateList();
            }
          }}
        />
        <button
          onClick={handleCreateList}
          disabled={creating || !newListName.trim()}
          className="btn-create"
        >
          {creating ? "创建中..." : "创建"}
        </button>
      </div>

      <div className="lists-container">
        {lists.length === 0 ? (
          <div className="empty-state">
            <p>还没有列表，创建一个吧</p>
          </div>
        ) : (
          lists.map((list) => (
            <div
              key={list.id}
              className={`list-item ${
                selectedListId === list.id ? "selected" : ""
              }`}
              onClick={() => onSelectList(list.id)}
            >
              {editingId === list.id ? (
                <div className="list-edit">
                  <input
                    type="text"
                    value={editingName}
                    onChange={(e) => setEditingName(e.target.value)}
                    maxLength={64}
                    className="list-name-input-inline"
                    autoFocus
                    onKeyPress={(e) => {
                      if (e.key === "Enter") {
                        handleRename(list.id);
                      }
                    }}
                    onClick={(e) => e.stopPropagation()}
                  />
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleRename(list.id);
                    }}
                    className="btn-save-inline"
                  >
                    ✓
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      setEditingId(null);
                    }}
                    className="btn-cancel-inline"
                  >
                    ✕
                  </button>
                </div>
              ) : (
                <>
                  <div className="list-info">
                    <span className="list-name">{list.name}</span>
                    <span className="fund-count">
                      {list.fund_codes.length}只基金
                    </span>
                  </div>
                  <div className="list-actions">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        setEditingId(list.id);
                        setEditingName(list.name);
                      }}
                      className="btn-action"
                      title="重命名"
                    >
                      ✏️
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDelete(list.id, list.name, list.fund_codes.length);
                      }}
                      className="btn-action"
                      title="删除"
                    >
                      🗑️
                    </button>
                  </div>
                </>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};
