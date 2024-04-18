# Assignment: Designing a Recommendation System for a Short Video Platform

## Overview

In this assignment, you are required to design a recommendation system for a short video platform. The goal of your recommendation system is to suggest videos to users that they are likely to watch, like, and engage with based on their previous interactions and similar user behaviours.

## Heads up

Writing code isn’t a requirement. We only expect you to present an approach for the task. Slides would save time during the interview and would show some commitment leading to some brownie points.

## Objectives

- Design a pipeline for personalised recommendations on a short video platform, beginning with a user's request and culminating in the delivery of a ranked list of the top-k videos from millions of video candidates.

## Requirements

- **Data Collection:** Raw features that you would want to log like user data (e.g., user ID, demographics), video metadata (e.g., video ID, tags, duration), and user interactions (e.g., likes, watches, shares).
- **Feature Engineering:** Data preprocessing steps to clean and prepare your data for modelling.
- **Model Development:** Develop and train a recommendation model using methods like collaborative filtering, content-based filtering, or a hybrid method.
- **Evaluation:** Implement metrics such as precision@k, recall@k, and F1-score to evaluate your model offline and also mention the metrics that we could track for online AB testing.
- **Real-time Recommendation:** Design a system that can update recommendations in real-time/batch wise based on new user interactions.
- **Scheduling and Monitoring:** Explain how to schedule the data preparation, model train etc. and monitor the pipeline for drift.
- **Databases:** You are expected to mention the database which you would like to use for each step.
- **Scalability:** Approach should be scalable for millions of users and videos.
