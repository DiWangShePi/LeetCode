# 355. Design Twitter

### Description

Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and is able to see the 10 most recent tweets in the user's news feed.

Implement the Twitter class:

- Twitter() Initializes your twitter object.
- void postTweet(int userId, int tweetId) Composes a new tweet with ID tweetId by the user userId. Each call to this function will be made with a unique tweetId.
- List<Integer> getNewsFeed(int userId) Retrieves the 10 most recent tweet IDs in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user themself. Tweets must be ordered from most recent to least recent.
- void follow(int followerId, int followeeId) The user with ID followerId started following the user with ID followeeId.
- void unfollow(int followerId, int followeeId) The user with ID followerId started unfollowing the user with ID followeeId.

### Example 

###### Example I

```
Input
["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
[[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
Output
[null, null, [5], null, null, [6, 5], null, [5]]

Explanation
Twitter twitter = new Twitter();
twitter.postTweet(1, 5); // User 1 posts a new tweet (id = 5).
twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
twitter.follow(1, 2);    // User 1 follows user 2.
twitter.postTweet(2, 6); // User 2 posts a new tweet (id = 6).
twitter.getNewsFeed(1);  // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
twitter.unfollow(1, 2);  // User 1 unfollows user 2.
twitter.getNewsFeed(1);  // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
```

### Solution

我一开始的想法是用一个列表顺序的存储所有推文，再用一个字典存储推文和发推的关系，这样便于维护时间顺序。

但问题在于，每一次查询需要遍历所有推文，而其中可能很多是不相关的。

> 暴力解法

```c++
class Twitter {
public:
    Twitter() {}
    
    void postTweet(int userId, int tweetId) {
        tweets.push_back(tweetId);
        tweets_belong[tweetId] = userId;
    }
    
    vector<int> getNewsFeed(int userId) {
        unordered_set<int> follower = following[userId];
        vector<int> an;
        for (int i = tweets.size() - 1; i > -1; i--) {
            int t = tweets[i];
            int belong_to = tweets_belong[t];

            if (belong_to == userId) an.push_back(t);
            if (follower.count(belong_to)) an.push_back(t);

            if (an.size() == 10) break;
        }
        return an;
    }
    
    void follow(int followerId, int followeeId) {
        if (followerId != followeeId) {
            following[followerId].insert(followeeId);
        }
    }
    
    void unfollow(int followerId, int followeeId) {
        following[followerId].erase(followeeId);
    }

private:
    vector<int> tweets;
    unordered_map<int, int> tweets_belong;
    unordered_map<int, unordered_set<int>> following;
};

/**
 * Your Twitter object will be instantiated and called as such:
 * Twitter* obj = new Twitter();
 * obj->postTweet(userId,tweetId);
 * vector<int> param_2 = obj->getNewsFeed(userId);
 * obj->follow(followerId,followeeId);
 * obj->unfollow(followerId,followeeId);
 */
```

我们可以为推文定制时间结构，用时间戳记录推文进入的时间。查询的时候获取自身推文和所有的关注者推文，放入堆中逐个取出即可

```c++
class Twitter {
private:
    struct Tweet {
        int id;
        int time;
        Tweet(int id, int time) : id(id), time(time) {}
    };
    
    int time;
    unordered_map<int, vector<Tweet>> userTweets;
    unordered_map<int, unordered_set<int>> following;
    unordered_map<int, int> tweetTimes;

public:
    Twitter() : time(0) {}
    
    void postTweet(int userId, int tweetId) {
        userTweets[userId].emplace_back(tweetId, time);
        tweetTimes[tweetId] = time++;
    }
    
    vector<int> getNewsFeed(int userId) {
        priority_queue<pair<int, int>> maxHeap;
        
        for (const Tweet& tweet : userTweets[userId]) {
            maxHeap.push({tweet.time, tweet.id});
        }
        
        for (int followeeId : following[userId]) {
            if (userTweets.count(followeeId)) {
                for (const Tweet& tweet : userTweets[followeeId]) {
                    maxHeap.push({tweet.time, tweet.id});
                }
            }
        }
        
        vector<int> feed;
        while (!maxHeap.empty() && feed.size() < 10) {
            feed.push_back(maxHeap.top().second);
            maxHeap.pop();
        }
        
        return feed;
    }
    
    void follow(int followerId, int followeeId) {
        if (followerId != followeeId) {
            following[followerId].insert(followeeId);
        }
    }
    
    void unfollow(int followerId, int followeeId) {
        following[followerId].erase(followeeId);
    }
};
```
