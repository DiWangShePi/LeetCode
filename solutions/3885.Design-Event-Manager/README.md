# 3885. Design Event Manager

**Tags:** Heap, Hash Map

### Description

You are given an initial list of events, where each event has a unique eventId and a priority.

Implement the EventManager class:

- EventManager(int[][] events) Initializes the manager with the given events, where events[i] = [eventIdi, priority​​​​​​​i].
- void updatePriority(int eventId, int newPriority) Updates the priority of the active event with id eventId to newPriority.
- int pollHighest() Removes and returns the eventId of the active event with the highest priority. If multiple active events have the same priority, return the smallest eventId among them. If there are no active events, return -1.
An event is called active if it has not been removed by pollHighest().

### Example

###### Example I

> Input:
> ["EventManager", "pollHighest", "updatePriority", "pollHighest", "pollHighest"]
> [[[[5, 7], [2, 7], [9, 4]]], [], [9, 7], [], []]
> Output:
> [null, 2, null, 5, 9]
> Explanation
> EventManager eventManager = new EventManager([[5,7], [2,7], [9,4]]); // Initializes the manager with three events
> eventManager.pollHighest(); // both events 5 and 2 have priority 7, so return the smaller id 2
> eventManager.updatePriority(9, 7); // event 9 now has priority 7
> eventManager.pollHighest(); // remaining highest priority events are 5 and 9, return 5
> eventManager.pollHighest(); // return 9

###### Example Ii

> Input:
> ["EventManager", "pollHighest", "pollHighest", "pollHighest"]
> [[[[4, 1], [7, 2]]], [], [], []]
> Output:
> [null, 7, 4, -1]
> Explanation
> EventManager eventManager = new EventManager([[4,1], [7,2]]); // Initializes the manager with two events
> eventManager.pollHighest(); // return 7
> eventManager.pollHighest(); // return 4
> eventManager.pollHighest(); // no events remain, return -1

### Solution

用堆存储事件，根据权重构建大顶堆。

但更新时不能从堆里面更新，所以用一个额外的字典存储事件和版本号，堆放事件、权重和版本号构成的三元组。拿出事件的时候检查是不是最新版的。

```c++
struct CompareBySecond {
    bool operator()(const std::vector<int>& a, const std::vector<int>& b) {
        return a[1] < b[1] || (a[1] == b[1] && a[0] > b[0]);  
    }
};

class EventManager {
    unordered_map<int, int> dict;
    priority_queue<vector<int>, vector<std::vector<int>>, CompareBySecond> pq;
    
public:
    EventManager(vector<vector<int>>& events) {
        for (vector<int>& event : events) {
            pq.push({event[0], event[1], 0});
            dict[event[0]] = 0;
        }
    }
    
    void updatePriority(int eventId, int newPriority) {
        dict[eventId]++;
        pq.push({eventId, newPriority, dict[eventId]});
    }
    
    int pollHighest() {
        if (pq.empty()) return -1;

        vector<int> current = pq.top();
        while (current[2] < dict[current[0]]) {
            pq.pop();
            if (pq.empty()) return -1;
            else current = pq.top();
        }
        int an = pq.top()[0];
        pq.pop();
        return an;
    }
};

/**
 * Your EventManager object will be instantiated and called as such:
 * EventManager* obj = new EventManager(events);
 * obj->updatePriority(eventId,newPriority);
 * int param_2 = obj->pollHighest();
 */
```
