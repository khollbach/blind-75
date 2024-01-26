/*
 * @lc app=leetcode id=133 lang=c
 *
 * [133] Clone Graph
 */

struct Node {
    int val;
    int numNeighbors;
    struct Node** neighbors;
};

// @lc code=start
#include <stdbool.h>
#include <malloc.h>
#include <assert.h>

typedef struct Node Node;

typedef struct Seen {
    int len;
    bool *seen;
} Seen;

Node *clone(Node *start);
void clone_dfs(Node *curr, Seen *seen, Node **clones);
int num_nodes(Node *start);
void num_nodes_dfs(Node *curr, Seen *seen);
Seen seen_new();
Seen seen_with_capacity(int n);
void seen_free(Seen self);
void seen_insert(Seen *self, int idx);
bool seen_seen(Seen *self, int idx);
int seen_count(Seen *self);
int max(int a, int b);

struct Node *cloneGraph(struct Node *s) {
    return clone(s);
}

Node *clone(Node *start) {
    if (!start) return NULL;

    int n = num_nodes(start);

    Node **clones = malloc((n + 1) * sizeof(Node *)); // 1-based indices
    for (int i = 1; i <= n; i++) {
        clones[i] = malloc(sizeof(Node));
        clones[i]->val = i;
    }

    Seen seen = seen_with_capacity(n);
    clone_dfs(start, &seen, clones);
    seen_free(seen);

    Node *out = clones[start->val];
    free(clones);
    return out;
}

void clone_dfs(Node *curr, Seen *seen, Node **clones) {
    if (seen_seen(seen, curr->val)) {
        return;
    }
    seen_insert(seen, curr->val);

    clones[curr->val]->numNeighbors = curr->numNeighbors;
    clones[curr->val]->neighbors = malloc(curr->numNeighbors * sizeof(Node *));

    for (int i = 0; i < curr->numNeighbors; i++) {
        clones[curr->val]->neighbors[i] = clones[curr->neighbors[i]->val];
        clone_dfs(curr->neighbors[i], seen, clones);
    }
}

int num_nodes(Node *start) {
    Seen seen = seen_new();
    num_nodes_dfs(start, &seen);
    seen_free(seen);
    return seen_count(&seen);
}

void num_nodes_dfs(Node *curr, Seen *seen) {
    if (seen_seen(seen, curr->val)) {
        return;
    }
    seen_insert(seen, curr->val);
    for (int i = 0; i < curr->numNeighbors; i++) {
        num_nodes_dfs(curr->neighbors[i], seen);
    }
}

Seen seen_new() {
    return seen_with_capacity(0);
}

Seen seen_with_capacity(int n) {
    Seen seen = { .len = n, .seen = calloc(n, sizeof(bool)) };
    return seen;
}

void seen_free(Seen self) {
    free(self.seen);
}

void seen_insert(Seen *self, int idx) {
    if (idx >= self->len) {
        int new_len = max(idx + 1, self->len * 2);
        self->seen = realloc(self->seen, new_len * sizeof(bool));
        for (int i = self->len; i < new_len; i++) {
            self->seen[i] = false;
        }
        self->len = new_len;
    }
    self->seen[idx] = true;
}

bool seen_seen(Seen *self, int idx) {
    assert(idx >= 0);
    if (idx >= self->len) {
        return false;
    }
    return self->seen[idx];
}

int seen_count(Seen *self) {
    int count = 0;
    for (int i = 0; i < self->len; i++) {
        count += self->seen[i];
    }
    return count;
}

int max(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}
// @lc code=end

int main() {
    int n = 4;
    Node *nodes = malloc(n * sizeof(Node));
    for (int i = 0; i < n; i++) {
        nodes[i].val = i + 1;
        nodes[i].numNeighbors = 2;
        nodes[i].neighbors = malloc(2 * sizeof(Node *));
    }
    nodes[0].neighbors[0] = nodes + 1; nodes[0].neighbors[1] = nodes + 3;
    nodes[1].neighbors[0] = nodes + 0; nodes[1].neighbors[1] = nodes + 2;
    nodes[2].neighbors[0] = nodes + 1; nodes[2].neighbors[1] = nodes + 3;
    nodes[3].neighbors[0] = nodes + 0; nodes[3].neighbors[1] = nodes + 2;

    Node *c = clone(&nodes[0]);
    (void) c;

    // todo: free both copies of the graph...
}
